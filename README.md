# MTG Deck Analyzer

A program to run analyses on MTG decks by simulating beginning game draws / mulligans and rating each run.

## Usage (Development)

Currently, the Rust code is not connected to the React application. Eventually, once the library is closer to completion, they will be connected.

To run everything concurrently:

```bash
yarn
yarn dev
```

Alternatively, you may run the frontend and backend separately:

### React Frontend

To run the React application:

```bash
yarn dev:frontend
```

### Spring Backend

To run the Spring backend, ensure you have at least Java 1.8 and Maven 3.6.3 installed on your machine. You may run the following from the project root:

```bash
yarn dev:backend
```

There is no hot-reload for the Java Spring backend at the moment, I am new to the framework and am unsure if this feature even exists. If it does I will be sure to add that into the configuration, however at the moment any changes made to the Java source will require a restart of process. As such, running the two packages separately is my go-to method for bootup.

## Documentation

### Rust

To run the Rust library testing suite:

```bash
cd packages/rust
cargo test --release
```

TODO: document lib and add this information into the readme

### Java Spring Backend

#### Entities

TODO: add this information into the readme

#### Routes

TODO: add this information into the readme

### React Frontend

The React frontend is written in TypeScript and uses Tailwind CSS for the styling of components.

TODO: add more information into the readme
