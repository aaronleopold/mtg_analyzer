# MTG Deck Analyzer

A program to run analyses on MTG decks by simulating beginning game draws / mulligans and rating each run.

## Getting Started

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

This will run the suite of tests. Please note that these tests are computationally intense and will require some decent CPU power.

### Spring Backend

To run the Spring backend, ensure you have at least Java 1.8 and Maven 3.6.3 installed on your machine. You may run the following from the project root:

```bash
yarn dev:backend
```

There is no hot-reload for the Java Spring backend at the moment, I am new to the framework and am unsure if this feature even exists. If it does I will be sure to add that into the configuration, however at the moment any changes made to the Java source will require a restart of process. As such, running the two packages separately is my go-to method for bootup

## Testing

To run the Rust library testing suite:

```bash
yarn test:rust
```

## Documentation

TODO: create this -> see [wiki](https://github.com/aaronleopold/mtg_analyzer/wiki) for more detailed documentation

TODO: document lib and add this information into the readme

### Java Spring Backend

#### Entities

TODO: add this information into the readme

#### Routes

TODO: add this information into the readme

### React Frontend

The React frontend is written in TypeScript and uses Tailwind CSS for the styling of components.

TODO: add more information into the readme
