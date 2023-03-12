/** @type {import('tailwindcss').Config} */
const defaultTheme = require('tailwindcss/defaultTheme')
module.exports = {
  content: ['./src/**/*.{html,ts,svelte}'],
  theme: {
    extend: {},
    colors: {
      'silver': '#CCCCCC',
      'white': '#FFFFFF',
      'black': '#000000',
      'transparent': 'transparent',
    },
    fontFamily: {
      'sans': ['Inter', ...defaultTheme.fontFamily.sans]
    }
  },
  plugins: [],
}
