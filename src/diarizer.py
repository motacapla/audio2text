import os
import whisper
from pyannote.audio import Pipeline
from pyannote.audio import Audio

pipeline = Pipeline.from_pretrained(
    "pyannote/speaker-diarization-3.1",
    use_auth_token=os.environ.get("HF_AUTH_TOKEN"),
)
audio = Audio(sample_rate=16000, mono=True)
model = whisper.load_model("large-v3-turbo")


def diarize(audio_file: str):
    diarization = pipeline(audio_file)
    results = []
    for segment, _, speaker in diarization.itertracks(yield_label=True):
        waveform, _ = audio.crop(audio_file, segment)
        text = model.transcribe(waveform.squeeze().numpy())["text"]
        print(f"[{segment.start:03.1f}s - {segment.end:03.1f}s] {speaker}: {text}")
        results.append(
            {
                "start": segment.start,
                "end": segment.end,
                "speaker": speaker,
                "text": text,
            }
        )
    return {"conversation": results}
