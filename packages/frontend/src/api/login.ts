import api from '.';

export default async function login(username: string, password: string) {
  console.log(username, password);

  const token = await api
    .post('/user/login', { username, password })
    .then((res) => res.jwtToken)
    .catch((err) => {
      console.log(err);
      return null;
    });

  console.log(token);

  // TODO: store this token in svelte store!
}
