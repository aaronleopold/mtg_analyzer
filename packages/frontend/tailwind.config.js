const defaultTheme = require('tailwindcss/defaultTheme');

module.exports = {
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter var', ...defaultTheme.fontFamily.sans],
      },
    },
  },
  variants: {
    backgroundColor: ['responsive', 'hover', 'focus', 'odd'],
    borderWidth: ['responsive', 'last', 'hover', 'focus'],
    margin: ['responsive', 'last'],
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
    require('@tailwindcss/aspect-ratio'),
    require('tailwind-caret-color'),
  ],
  purge: ['./src/**/*.tsx', './src/**/*.ts', './src/**/*.html'],
};
