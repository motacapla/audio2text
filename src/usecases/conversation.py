import sys
import os
import playsound

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "../..")))

from src.entities import diarizer
from src.entities import local_llm
from src.entities import tts
from src.entities import record

if __name__ == "__main__":
    question_input_path = "./conversation_history/question.wav"
    response_output_path = "./conversation_history/answer.wav"
    record.record_audio(output_path=question_input_path)

    json_dict = diarizer.diarize(question_input_path)
    print(json_dict)

    response = local_llm.ask_ollama(json_dict["conversation"])
    print(response)

    response_output_path = tts.generate_speech(
        response, output_path=response_output_path, verbose=True
    )

    playsound.playsound(response_output_path)
