import os
import requests


def ask_ollama(texts: list[str], max_tokens=10000):
    prompt = "以下のテキストは音声から抽出したものです。相手の名前について言及せずに、質問について回答してください。\n"
    prompt += "\n".join(texts)
    """
    Ollamaに質問して回答を取得する関数

    Args:
        texts (list): Ollamaに送信する質問やプロンプト
        max_tokens (int, optional): 生成する回答の最大トークン数。デフォルトは1000。

    Returns:
        str: Ollamaからの回答
    """
    url = "http://localhost:11434/api/generate"

    data = {
        "model": "elyza:jp8b",  # 使用するモデル名
        "prompt": prompt,
        "max_tokens": max_tokens,
        "stream": False,
    }

    response = requests.post(url, json=data)

    if response.status_code != 200:
        raise Exception(f"failed to ask_ollama: {response.status_code}")

    result = response.json()
    return result.get("response", "")


if __name__ == "__main__":
    print(
        ask_ollama(
            "[3.7s - 10.0s] SPEAKER_00: 日本の春ってなんでこんなに寒いんですか?3月21日なんですけど、なんか全然気温が上がらないんです。",
            max_tokens=1000,
        )
    )
