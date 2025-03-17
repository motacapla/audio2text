FROM python:3.12

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ffmpeg \
    libsndfile1 \
    git \
    && rm -rf /var/lib/apt/lists/*

COPY requirements.txt /app/
COPY .env /app/

RUN pip install -r requirements.txt

COPY src_py /app/src_py
COPY data /app/data

RUN chmod +x /app/src_py/main.py

CMD ["python", "src_py/main.py"]