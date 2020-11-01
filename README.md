# MTG Deck Analyzer

A program to run analyses on MTG decks by simulating beginning game draws / mulligans and rating each run.

## Usage (Development)

Currently, the Rust code is not connected to the React application. Eventually, once the library is closer to completion, they will be connected.

To run the React application:

```bash
$ cd mtg_benchmarker
$ yarn
$ yarn install:all
$ yarn dev
```

To run the frontend or backend separately, the corresponding commands exist:

```bash
$ yarn dev:frontend
$ yarn dev:backend
```

To run the Rust library testing suite:

```bash
$ cd mtg_benchmarker/packages/rust
$ cargo build
$ cargo run --release
```
