use std::process::Command;
use std::path::Path;

fn convert_mp4_to_mp3(input_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(input_file).exists() {
        return Err(format!("not found: {}", input_file).into());
    }

    let status = Command::new("ffmpeg")
        .args(&["-i", input_file, "-q:a", "0", "-map", "a", output_file])
        .status()?;

    if !status.success() {
        return Err("failed to convert".into());
    } else {
        Ok(())
    }
}

fn main() {
    let input_file = "data/input.mp4";
    let output_file = "data/output.mp3";

    match convert_mp4_to_mp3(input_file, output_file) {
        Ok(_) => println!("success to convert: {}", output_file),
        Err(e) => eprintln!("an error has occurred: {}", e),
    }
}