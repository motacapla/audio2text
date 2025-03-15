use anyhow::{anyhow, Result};
use std::fs::File;
use std::path::{Path};
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use hound::{WavSpec, WavWriter};
use rubato::{FftFixedIn, Resampler};

fn main() -> Result<()> {    
    let m4a_files = std::fs::read_dir("./data")?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "m4a" {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    
    if m4a_files.is_empty() {
        println!("m4aファイルが見つかりませんでした");
        return Ok(());
    }
    
    for input_file in m4a_files {
        let output_file = input_file.to_string_lossy().to_string().replace(".m4a", ".wav");
        let output_path = Path::new(&output_file);
        convert_m4a_to_wav(&input_file, output_path)?;
    }
    
    Ok(())
}

fn convert_m4a_to_wav(input_path: &Path, output_path: &Path) -> Result<()> {
    // 入力ファイルを開く
    let file = File::open(input_path)?;
    
    // ファイルの種類を推測するためのヒントを作成
    let mut hint = Hint::new();
    if let Some(extension) = input_path.extension() {
        if let Some(ext_str) = extension.to_str() {
            hint.with_extension(ext_str);
        }
    }
    
    // メディアソースストリームを作成
    let media_source = MediaSourceStream::new(Box::new(file), Default::default());
    
    // フォーマットリーダーをプローブ
    let format_opts = FormatOptions::default();
    let metadata_opts = MetadataOptions::default();
    let decoder_opts = DecoderOptions::default();
    
    // フォーマットをプローブ
    let probed = symphonia::default::get_probe()
        .format(&hint, media_source, &format_opts, &metadata_opts)
        .map_err(|_| anyhow!("ファイルフォーマットを認識できません。m4aファイルであることを確認してください。"))?;
    
    // フォーマットリーダーとデフォルトトラックを取得
    let mut format = probed.format;
    let track = format
        .default_track()
        .ok_or_else(|| anyhow!("デフォルトトラックが見つかりません"))?;
    
    // デコーダーを作成
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decoder_opts)?;
    
    // 元のサンプルレートを取得
    let original_sample_rate = track.codec_params.sample_rate.unwrap_or(44100);
    
    // 出力用のサンプルレートを設定（一般的なWAVファイルのサンプルレート）
    let output_sample_rate = original_sample_rate; // 同じサンプルレートを使用
    
    // チャンネル数を取得（デフォルトは1）
    let channels = match track.codec_params.channels {
        Some(channels) => {
            let count = channels.count();
            if count == 0 {
                return Err(anyhow!("チャンネル数が0です"));
            }
            count as u16
        },
        None => 1, // デフォルトはモノラル
    };
    
    // サンプルレートが異なる場合はリサンプラーを作成
    let need_resampling = original_sample_rate != output_sample_rate;
    let mut resampler = if need_resampling {
        // リサンプラーの設定
        let resampler = FftFixedIn::<f32>::new(
            original_sample_rate as usize,
            output_sample_rate as usize,
            4096, // バッファサイズ
            2,    // オーバーラップ
            channels as usize,
        ).map_err(|e| anyhow!("リサンプラーの作成に失敗しました: {}", e))?;
        Some(resampler)
    } else {
        None
    };
    
    let spec = WavSpec {
        channels,
        sample_rate: output_sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    // WAVライターを作成
    let mut wav_writer = WavWriter::create(output_path, spec)?;
    
    // デコードしたオーディオデータを保存するバッファ
    let mut audio_buffer: Vec<Vec<f32>> = (0..channels as usize).map(|_| Vec::new()).collect();
    
    // パケットを読み込んでデコード
    while let Ok(packet) = format.next_packet() {
        // パケットをデコード
        match decoder.decode(&packet) {
            Ok(decoded) => {
                // デコードされたオーディオバッファを取得
                match decoded {
                    AudioBufferRef::F32(buf) => {
                        // バッファのチャンネル数を取得
                        let buf_channels = buf.spec().channels.count();
                        let frames = buf.frames();
                        
                        // チャンネルごとにサンプルを収集
                        for channel in 0..std::cmp::min(buf_channels, channels as usize) {
                            for frame in 0..frames {
                                audio_buffer[channel].push(buf.chan(channel)[frame]);
                            }
                        }
                    },
                    AudioBufferRef::U16(buf) => {
                        let buf_channels = buf.spec().channels.count();
                        let frames = buf.frames();
                        
                        // チャンネルごとにサンプルを収集（U16をf32に変換）
                        for channel in 0..std::cmp::min(buf_channels, channels as usize) {
                            for frame in 0..frames {
                                let sample = buf.chan(channel)[frame] as f32 / 32768.0;
                                audio_buffer[channel].push(sample);
                            }
                        }
                    },
                    AudioBufferRef::S16(buf) => {
                        let buf_channels = buf.spec().channels.count();
                        let frames = buf.frames();
                        
                        // チャンネルごとにサンプルを収集（S16をf32に変換）
                        for channel in 0..std::cmp::min(buf_channels, channels as usize) {
                            for frame in 0..frames {
                                let sample = buf.chan(channel)[frame] as f32 / 32768.0;
                                audio_buffer[channel].push(sample);
                            }
                        }
                    },
                    AudioBufferRef::S32(buf) => {
                        let buf_channels = buf.spec().channels.count();
                        let frames = buf.frames();
                        
                        // チャンネルごとにサンプルを収集（S32をf32に変換）
                        for channel in 0..std::cmp::min(buf_channels, channels as usize) {
                            for frame in 0..frames {
                                let sample = buf.chan(channel)[frame] as f32 / 2147483648.0;
                                audio_buffer[channel].push(sample);
                            }
                        }
                    },
                    _ => {
                        return Err(anyhow!("未対応のオーディオフォーマットです"));
                    }
                }
            }
            Err(err) => {
                eprintln!("デコードエラー: {}", err);
                continue;
            }
        }
    }
    
    // 全てのオーディオデータを処理
    if need_resampling && !audio_buffer.is_empty() && !audio_buffer[0].is_empty() {
        if let Some(resampler) = &mut resampler {
            // リサンプリングを実行
            let resampled_audio = resampler.process(&audio_buffer, None)
                .map_err(|e| anyhow!("リサンプリングに失敗しました: {}", e))?;
            
            // リサンプリングされたデータをWAVファイルに書き込む
            let num_frames = resampled_audio[0].len();
            for frame in 0..num_frames {
                for channel in 0..channels as usize {
                    let sample = resampled_audio[channel][frame];
                    let sample_i16 = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
                    wav_writer.write_sample(sample_i16)?;
                }
            }
        }
    } else if !audio_buffer.is_empty() && !audio_buffer[0].is_empty() {
        // リサンプリングが不要な場合は直接書き込む
        let num_frames = audio_buffer[0].len();
        for frame in 0..num_frames {
            for channel in 0..channels as usize {
                let sample = audio_buffer[channel][frame];
                let sample_i16 = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
                wav_writer.write_sample(sample_i16)?;
            }
        }
    } else {
        return Err(anyhow!("オーディオデータが取得できませんでした"));
    }
    
    wav_writer.finalize()?;
    
    Ok(())
}