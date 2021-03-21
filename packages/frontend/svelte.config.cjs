const sveltePreprocess = require('svelte-preprocess');
const _static = require('@sveltejs/adapter-static');

/** @type {import('@sveltejs/kit').Config} */
module.exports = {
  preprocess: sveltePreprocess({
    postcss: true,
  }),
  kit: {
    adapter: _static(),
    target: '#svelte',
  },
};
