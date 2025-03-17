import subprocess


# ffmpeg -i input.m4a -acodec pcm_s16le -ar 16000 -ac 1 output.wav
def convert_m4a_to_wav(input_file: str, output_file: str):
    subprocess.run(
        [
            "ffmpeg",
            "-i",
            input_file,
            "-acodec",
            "pcm_s16le",
            "-ar",
            "16000",
            "-ac",
            "1",
            output_file,
        ]
    )
