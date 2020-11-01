import { SqlHighlighter } from '@mikro-orm/sql-highlighter';
import { Options } from 'mikro-orm';
import { TsMorphMetadataProvider } from '@mikro-orm/reflection';

// module.exports = {
//   type: 'sqlite',
//   database: 'database.sqlite',
//   synchronize: true,
//   // logging: false,
//   entities: ['src/entities/**/*.ts'],
//   migrations: ['src/migration/**/*.ts'],
//   subscribers: ['src/subscriber/**/*.ts'],
//   cli: {
//     entitiesDir: 'src/entities',
//     migrationsDir: 'src/migration',
//     subscribersDir: 'src/subscriber',
//   },
// };

const config: Options = {
  type: 'sqlite',
  dbName: 'database.db',
  entitiesTs: ['./src/entities'],
  entities: ['./dist/entities'],
  highlighter: new SqlHighlighter(),
  debug: true,
  metadataProvider: TsMorphMetadataProvider,
};

export default config;
