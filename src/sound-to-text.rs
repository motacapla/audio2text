use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use std::io::Write;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model_path = "model/ggml-large-v3.bin";
    let language = "ja";

    let params = WhisperContextParameters::default();    
    let ctx = WhisperContext::new_with_params(model_path, params)
        .expect("モデルの読み込みに失敗しました");

    let host = cpal::default_host();
    let device = host.default_input_device().expect("入力デバイスが見つかりません");
    let config = device.default_input_config().expect("デフォルト設定が見つかりません");
    
    println!("デバイス: {}", device.name().unwrap_or_default());
    println!("設定: {:?}", config);
    
    let audio_buffer = Arc::new(Mutex::new(Vec::<f32>::new()));
    let audio_buffer_clone = audio_buffer.clone();
    
    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _| {
            let mut buffer = audio_buffer_clone.lock().unwrap();
            buffer.extend_from_slice(data);
        },
        move |err| {
            eprintln!("録音エラー: {:?}", err);
        },
        None,
    )?;
    
    stream.play()?;
    println!("録音中... Ctrl+Cで停止");
    
    let _whisper_thread = thread::spawn(move || {
        let mut state = ctx.create_state().expect("状態の作成に失敗しました");
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_no_speech_thold(0.3);
        params.set_temperature(0.0);

        params.set_language(Some(language));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        
        let process_interval = Duration::from_secs(3);
        
        loop {
            thread::sleep(process_interval);
            
            // バッファからデータを取得
            let mut buffer = audio_buffer.lock().unwrap();
            if buffer.len() < 8000 {  // 少なすぎる場合はスキップ
                continue;
            }
            
            let samples = buffer.clone();
            buffer.clear();
            drop(buffer);
            
            // ステレオからモノラルへ変換
            let mono_samples = convert_stereo_to_mono(&samples);
            
            // サンプルレートの変換（必要に応じて）
            // ここでは簡略化のため省略
            
            process_audio(&mut state, &params, &mono_samples);
        }
    });
    
    // メインスレッドでCtrl+Cを待機
    loop {
        thread::sleep(Duration::from_millis(50));
    }
}

// ステレオからモノラルへの変換
fn convert_stereo_to_mono(stereo_samples: &[f32]) -> Vec<f32> {
    let mut mono = Vec::with_capacity(stereo_samples.len() / 2);
    let mut i = 0;
    while i + 1 < stereo_samples.len() {
        let mixed = (stereo_samples[i] + stereo_samples[i+1]) * 0.5;
        mono.push(mixed);
        i += 2;
    }
    mono
}

fn process_audio(state: &mut whisper_rs::WhisperState, params: &FullParams, samples: &[f32]) {
    match state.full(params.clone(), samples) {
        Ok(_) => {
            let num_segments = state.full_n_segments().expect("セグメント数の取得に失敗");
            let mut text = String::new();
            if num_segments > 0 {
                println!("文字起こし結果:");
                for i in 0..num_segments {
                    let segment = state.full_get_segment_text(i).expect("セグメントの取得に失敗");
                    let start = state.full_get_segment_t0(i).expect("開始時間の取得に失敗");
                    let end = state.full_get_segment_t1(i).expect("終了時間の取得に失敗");
                    println!("[{} - {}]: {}", start, end, segment);
                    text += &trim_noise_text(segment);
                }
                println!("-------------------");
                let mut file = if std::path::Path::new("data/text.txt").exists() {
                    let mut file = std::fs::OpenOptions::new()
                        .append(true)
                        .open("data/text.txt")
                        .expect("ファイルを開けませんでした");
                    file
                } else {
                    std::fs::File::create("data/text.txt").expect("ファイルを作成できませんでした")
                };
                file.write_all(text.as_bytes()).expect("ファイルへの書き込みに失敗しました");
            }
        },
        Err(e) => {
            eprintln!("Whisper処理エラー: {:?}", e);
        }
    }
}

fn trim_noise_text(text: String) -> String {
    let mut result = text;
    result.push_str("\n");
    result = result.replace("ご視聴ありがとうございました", "");
    result = result.replace("ありがとうございました", "");
    result
}