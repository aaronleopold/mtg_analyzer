import { derived, writable } from 'svelte/store';

// TODO: should I really be storing the jwt in store? if I don't persist
// it then you just have to relogin, but local and session storage is not safe

export type User = {
  username: string;
  // firstName: string;
  // lastName: string;
  // email: string;
  // TODO: more
};

export type Session = {
  user: User | null;
  jwtToken: string | null;
};

export const authStore = (function () {
  const { subscribe, set, update } = writable<Session>({
    user: null,
    jwtToken: null,
  });

  return {
    update,
    subscribe,
    set,
    logUserIn: (user: User, jwtToken) => set({ user, jwtToken }),
  };
})();

export const loggedIn = derived(authStore, ($authStore) => {
  return $authStore.user !== null;
});
