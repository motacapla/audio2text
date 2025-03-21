import torch
from parler_tts import ParlerTTSForConditionalGeneration
from transformers import AutoTokenizer
import soundfile as sf
from rubyinserter import add_ruby

device = "cuda:0" if torch.cuda.is_available() else "cpu"

model_name = "2121-8/japanese-parler-tts-mini"

model = ParlerTTSForConditionalGeneration.from_pretrained(model_name).to(device)
prompt_tokenizer = AutoTokenizer.from_pretrained(
    model_name, subfolder="prompt_tokenizer"
)
description_tokenizer = AutoTokenizer.from_pretrained(
    model_name, subfolder="description_tokenizer"
)


def generate_speech(
    prompt,
    output_path="./conversation_history/answer.wav",
    description="少女の声で、明るくて楽しい雰囲気で、話しています。",
    verbose=False,
):
    """
    テキストから音声を生成する関数

    Args:
        prompt (str): 音声に変換するテキスト
        output_path (str, optional): 出力ファイルのパス。
        description (str, optional): 音声の特徴を指定する説明文。デフォルトは少女の声。

    Returns:
        str: 生成された音声ファイルのパス
    """
    prompt_with_ruby = add_ruby(prompt)
    input_ids = description_tokenizer(description, return_tensors="pt").input_ids.to(
        device
    )
    prompt_input_ids = prompt_tokenizer(
        prompt_with_ruby, return_tensors="pt"
    ).input_ids.to(device)

    if verbose:
        print("Generating speech...")

    generation = model.generate(input_ids=input_ids, prompt_input_ids=prompt_input_ids)
    audio_arr = generation.cpu().numpy().squeeze()
    sf.write(output_path, audio_arr, model.config.sampling_rate)

    return output_path


if __name__ == "__main__":
    text = """
    粉症の対策としては、まずマスクを着用し、目や鼻を保護することが大切です。また、外出から帰った後は服を着替え、洗濯物もすぐに洗うことで、家の中に花粉を持ち込まないようにします。室内ではエアコンや空気清浄機を使用し、定期的に掃除を行うと効果的です。
    """
    generate_speech(text, output_path="./conversation_history/answer.wav")
