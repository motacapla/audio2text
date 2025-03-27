import os
from dataclasses import replace
from pydub import AudioSegment
import tempfile
from tqdm import tqdm

import whisper
from pyannote.audio import Pipeline
from pyannote.audio import Audio

pipeline = Pipeline.from_pretrained(
    "pyannote/speaker-diarization-3.1",
    use_auth_token=os.environ.get("HF_AUTH_TOKEN"),
)
audio = Audio(sample_rate=16000, mono=True)
model = whisper.load_model("large-v3-turbo")


def diarize(filename: str):
    def flatten_list(lst: list):
        return [
            "# 名前: " + item["speaker"] + "\n# 内容: " + item["content"]
            for item in lst
        ]

    def diarize_internal(audio_file: str):
        diarization = pipeline(audio_file)
        results = []
        for segment, _, speaker in diarization.itertracks(yield_label=True):
            # 音声ファイルの長さを取得
            audio_duration = AudioSegment.from_wav(audio_file).duration_seconds
            
            # セグメントの終了時間がファイルの長さを超えていないか確認
            if segment.end > audio_duration:
                segment = replace(segment, end=audio_duration)
            
            # セグメントの開始時間と終了時間が有効かチェック
            if segment.start >= segment.end:
                continue
            
            try:
                waveform, _ = audio.crop(audio_file, segment)
                text = model.transcribe(waveform.squeeze().numpy(), fp16=False)["text"]
                print(f"[{segment.start:03.1f}s - {segment.end:03.1f}s] {speaker}: {text}")
                results.append(
                    {
                        "start": segment.start,
                        "end": segment.end,
                        "speaker": speaker,
                        "content": text,
                    }
                )
            except ValueError as e:
                print(f"セグメント [{segment.start:.2f}s - {segment.end:.2f}s] の処理中にエラーが発生しました: {e}")
                continue
                
        return results

    # 10秒ごとに音声ファイルを分割
    # audio_segment = AudioSegment.from_wav(filename)
    # duration_ms = len(audio_segment)
    # segment_ms = 10 * 1000

    # all_results = []
    # with tempfile.TemporaryDirectory() as temp_dir:
    #     for i in tqdm(range(0, duration_ms, segment_ms)):
    #         segment = audio_segment[i : i + segment_ms]
    #         temp_file = f"{temp_dir}/segment_{i}.wav"
    #         segment.export(temp_file, format="wav")

    #         try:
    #             segment_result = diarize_internal(temp_file)
    #             all_results.extend(segment_result)
    #         except (TypeError, ValueError) as e:
    #             print(f"セグメント {i} の処理中にエラーが発生しました: {e}")
    #             # セグメントが短すぎる場合などのエラーをスキップ
    #             continue

    all_results = []
    segment_result = diarize_internal(filename)
    all_results.extend(segment_result)

    # return {"conversation": flatten_list(results)}
    return {"conversation": (all_results)}


if __name__ == "__main__":
    import time
    import json

    start_time = time.time()

    filename = "./data/GMT20250305-060528_Recording.wav"

    result = diarize(filename)

    with open(filename.replace(".wav", ".json"), "w") as f:
        json.dump(result, f, ensure_ascii=False)
    end_time = time.time()

    print(f"処理時間: {end_time - start_time:.2f}秒")
