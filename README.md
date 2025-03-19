# audio2text
## Diarize audio file from zoom's record
### Prerequisites
#### input files
- create `data` directory
- put `.m4a` files in `data` directory

#### openai-whisper model
- register huggingface account and create access token
- agree and access repository : https://huggingface.co/openai/whisper-large-v3-turbo 
- create `.env` file in the root directory
- add `HUGGINGFACE_TOKEN` to `.env` file

### Run
```
$ docker-compose build && docker-compose up
```

### Result
- `.json` files will be created in `data` directory.
