const { tailwindExtractor } = require('tailwindcss/lib/lib/purgeUnusedStyles');

module.exports = {
  purge: {
    content: ['./src/**/*.html', './src/**/*.svelte'],
    options: {
      defaultExtractor: (content) => [
        ...tailwindExtractor(content),
        // Match Svelte class: directives (https://github.com/tailwindlabs/tailwindcss/discussions/1731)
        ...[...content.matchAll(/(?:class:)*([\w\d-/:%.]+)/gm)].map(
          ([_match, group, ..._rest]) => group
        ),
      ],
      keyframes: true,
    },
  },
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        dark: {
          900: 'rgb(25,25,25)',
          800: 'rgb(36,36,36)',
          700: 'rgb(40,40,40)',
          600: '#2D2D2D',
          500: 'rgb(50,50,50)',
          400: '#3d3d3d',
          300: '#b2b2b2',
          200: 'rgb(223,223,223)',
          twilight: 'rgb(30, 30, 30)',
          stacking: 'rgba(25,25,25,0.2)',
        },
      },
    },
  },
  variants: {},
  plugins: [require('tailwindcss-textshadow')],
};
