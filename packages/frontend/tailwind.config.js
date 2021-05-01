const { guessProductionMode } = require('@ngneat/tailwind');

process.env.TAILWIND_MODE = guessProductionMode() ? 'build' : 'watch';

module.exports = {
  prefix: '',
  mode: 'jit',
  purge: {
    content: ['./src/**/*.{html,ts,css,scss,sass,less,styl}'],
  },
  darkMode: 'class', // or 'media' or 'class'
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
    height: {
      '60vh': '60vh',
      '40vh': '40vh',
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
