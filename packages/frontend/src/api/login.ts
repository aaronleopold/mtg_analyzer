import api from '.';

export default async function login(username: string, password: string) {
  const token = await api
    .post('/user/login', { username, password })
    .then((res) => res.jwtToken)
    .catch((err) => {
      console.log(err);
      return null;
    });

  return token;
}
