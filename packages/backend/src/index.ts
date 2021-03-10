import Koa from 'koa';
import bodyParser from 'koa-bodyparser';
import session from 'koa-session';
import { ApolloServer } from 'apollo-server-koa';
import { AuthChecker, buildSchema } from 'type-graphql';
import { MikroORM } from 'mikro-orm';
import { KoaContext } from './types';
import { ApolloServerLoaderPlugin } from 'type-graphql-dataloader';
import bcrypt from 'bcryptjs';
import resolvers from './resolvers';

require('dotenv').config();

// TODO: make me work
async function seed() {
  const SALT = 12;

  const hashedPassword = await bcrypt.hash('dev', SALT);

  // const user = await User.create({
  //   name: 'Aaron Leopold',
  //   username: 'dev',
  //   email: 'dev@gmail.com',
  //   password: hashedPassword,
  // }).save();

  // await Deck.create({
  //   name: 'goblins',
  //   owner: user,
  // }).save();
}

// async function testing() {
//   const user = await User.findOneOrFail();

//   console.log(user);

//   console.log(await user.decks);
//   // goblins should fail to create here
//   try {
//     await Deck.create({
//       name: "goblins",
//       owner: user,
//     }).save();
//   } catch (e) {
//     if (e.routine === "_bt_check_unique") {
//       console.log(
//         "It worked, cannot create multiple decks with same name PER user"
//       );
//     } else {
//       console.log(e);
//     }
//   }
// }

// TODO: add context
// TODO: add cors

async function bootstrap() {
  const orm = await MikroORM.init();
  console.log(orm.em);

  const schema = await buildSchema({
    resolvers,
    emitSchemaFile: true,
    // authChecker, TODO:
  });

  const app = new Koa();

  app.keys = [process.env.COOKIE_SECRET!];

  const SESSION_CONFIG = {
    key: process.env.COOKIE_NAME!,
    maxAge: 86400000 * 14,
    autoCommit: true,
    overwrite: true,
    httpOnly: true,
    signed: true,
    rolling: true,
    renew: false,
  };

  app.use(session(SESSION_CONFIG, app));
  app.use(bodyParser());

  const server = new ApolloServer({
    schema,
    debug: true,
    uploads: true,
    plugins: [ApolloServerLoaderPlugin()],

    // TODO: type me
    async context({ ctx }): Promise<any> {
      return ctx.context;
    },
  });

  const cors = {
    origin: process.env.FRONTEND_URL!,
    credentials: true,
  };

  server.applyMiddleware({ app, cors, path: process.env.API_PATH! });

  app.listen(4000, () => {
    console.log('🚀 Apollo server running on http://localhost:4000');
  });
}

bootstrap();
