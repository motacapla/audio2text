# cookbook

## Diarize audio file from zoom's record
### Prerequisites
- create `data` directory
- rename m4a file to `input.m4a` and put it in `data` directory

### Run
$ cargo run --bin convert-m4a -- -i ./data/input.m4a
$ cargo run --bin diarizer