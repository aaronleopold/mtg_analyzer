# MTG Deck Analyzer

A program to run analyses on MTG decks by simulating beginning game draws / mulligans and rating each run.

## Usage (Development)
```bash
$ python3 manage.py tailwind install
$ python3 manage.py tailwind build
$ python3 manage.py runserver
```

Rust is not callable from python yet, so it will need to be built and ran separately
```bash
$ cd mtg_benchmarker
$ cargo build
$ cargo run --release
```
Running rust library manually will eventually just run the tests written for the benchmarking implementation
