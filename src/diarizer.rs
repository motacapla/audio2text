use std::process::Command;
use std::path::Path;
use std::fs;
use anyhow::{anyhow, Result};

fn run_diarizer(input_file: &str) -> Result<()> {
    if !Path::new(input_file).exists() {
        return Err(anyhow!("入力ファイルが見つかりません: {}", input_file));
    }

    let mut cmd = Command::new("python");
    cmd.arg("src_py/diarizer.py");
    // cmd.arg("--input").arg(input_file);

    let output = cmd.output()?;

    if output.status.success() {
        println!("diarizer.pyの実行が成功しました");
        if !output.stdout.is_empty() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("出力:\n{}", stdout);
        }
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow!("diarizer.pyの実行に失敗しました: {}", stderr))
    }
}

fn main() -> Result<()> {
    let wav_files = fs::read_dir("./data")?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "wav" {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    
    if wav_files.is_empty() {
        println!("./dataディレクトリにwavファイルが見つかりませんでした");
        return Ok(());
    }
    
    for input_path in wav_files {
        let input_file = input_path.to_str().unwrap();
        println!("処理中: {}", input_file);
        match run_diarizer(input_file) {
            Ok(_) => println!("成功: {}", input_file),
            Err(e) => eprintln!("エラー: {}", e),
        }
    }
    
    Ok(())
}