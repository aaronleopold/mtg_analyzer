import Koa from 'koa';

export type Session = {
  userID: string;
};

export type KoaContext = Koa.ParameterizedContext & {
  session: Session;
};
