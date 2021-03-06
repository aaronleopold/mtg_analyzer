# MTG Deck Analyzer

A program to run analyses on MTG decks by simulating beginning game draws / mulligans and rating each run.

## Getting Started

Currently, the Rust code is not connected to the web application. Eventually, once the library is closer to completion, they will be connected. ~~I recently moved the frontend over to using Svelte~~ I just migrated over to Angular from Svelte. There is ABSOLUTELY NOTHING wrong with svelte, this project is just a way for me to learn technologies I will be using in my upcoming internship this summer. I learned that I will be using Angular, so I changed the tech stack again (perhaps a few times too many :D!)

### Config

Currently you'll only need to set up `application.yml` for the Spring backend. It has the following structure:

```yml
# mtg_analyzer/packages/backend/src/main/resources/application.yml
jwt:
  secret: secretgoeshere
```

### Run

To run everything concurrently:

```bash
yarn
yarn dev
```

Alternatively, you may run the frontend and backend separately:

#### Angular Frontend

To run the Angular frontend:

```bash
yarn dev:frontend
```

#### Spring Backend

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

This will run the suite of tests. Please note that these tests are computationally intense and will require some decent CPU power.

## Documentation

TODO: create this -> see [wiki](https://github.com/aaronleopold/mtg_analyzer/wiki) for more detailed documentation

TODO: document lib and add this information into the readme

### Java Spring Backend

#### Entities

TODO: add this information into the readme

#### Routes

TODO: add this information into the readme

### Angular Frontend

The Angular frontend uses Tailwind CSS for the styling of components/pages.

TODO: add more information into the readme
