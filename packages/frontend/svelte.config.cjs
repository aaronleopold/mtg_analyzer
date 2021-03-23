const sveltePreprocess = require('svelte-preprocess');
const _static = require('@sveltejs/adapter-static');
const path = require('path');

/** @type {import('@sveltejs/kit').Config} */
module.exports = {
  preprocess: sveltePreprocess({
    postcss: true,
  }),
  kit: {
    adapter: _static(),
    target: '#svelte',
    vite: {
      // THIS DOES NOT CURRENTLY WORK !!
      resolve: {
        alias: [
          { find: '$api', replacement: path.resolve(__dirname, './src/api') },
        ],
      },
    },
  },
};
