import glob
import json
import sys
import os

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "../..")))

from src.entities import convert_m4a
from src.entities import diarizer

if __name__ == "__main__":
    m4a_files = glob.glob("data/*.m4a")
    for input_file in m4a_files:
        output_file = input_file.replace(".m4a", ".wav")
        convert_m4a.convert_m4a_to_wav(input_file, output_file)
        # input_file = "data/GMT20250303-063834_Recording.wav"
        results = diarizer.diarize(output_file)
        with open(input_file.replace(".m4a", ".json"), "w") as f:
            json.dump(results, f, ensure_ascii=False)
