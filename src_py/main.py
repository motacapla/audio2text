import glob
import json

import convert_m4a
import diarizer

if __name__ == "__main__":
    m4a_files = glob.glob("data/*.m4a")
    for input_file in m4a_files:
        output_file = input_file.replace(".m4a", ".wav")
        convert_m4a.convert_m4a_to_wav(input_file, output_file)
        results = diarizer.diarize(output_file)
        with open(input_file.replace(".m4a", ".json"), "w") as f:
            json.dump(results, f, ensure_ascii=False)
