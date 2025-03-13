use std::process::Command;
use std::path::Path;

fn convert_mp4_to_wav(input_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(input_file).exists() {
        return Err(format!("not found: {}", input_file).into());
    }

    let status = Command::new("ffmpeg")
        .args(&[
            "-i",
            input_file,
            "-vn",          // ビデオを無視（音声のみ）
            "-ar",
            "44100",        // サンプリングレート (44.1kHz)
            "-ac",
            "2",            // チャンネル数 (ステレオ)
            "-f",
            "wav",          // 出力形式
            output_file,
            "-y",           // ファイル上書き許可
        ])
        .status()?;

    if !status.success() {
        return Err("failed to convert".into());
    } else {
        Ok(())
    }
}

fn main() {
    let input_file = "data/input.mp4";
    let output_file = "data/output.wav";

    match convert_mp4_to_wav(input_file, output_file) {
        Ok(_) => println!("success to convert: {}", output_file),
        Err(e) => eprintln!("an error has occurred: {}", e),
    }
}