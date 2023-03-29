/** @type {import('tailwindcss').Config} */
const defaultTheme = require('tailwindcss/defaultTheme')
module.exports = {
  content: ['./src/**/*.{html,ts,svelte}'],
  theme: {
    extend: {
      colors: {
        'silver': '#CCCCCC',
        'white': '#FFFFFF',
        'black': '#000000',
        'transparent': 'transparent',
        'action': '#4C4C94',
        'link': '#1DC8D7',
        'currentIndicator': '#888888',
        'separator': '#444444',
        'you': '#777777',
      }
    },
    fontFamily: {
      'sans': ['Inter', ...defaultTheme.fontFamily.sans]
    }
  },
  plugins: [
    require('tailwind-scrollbar')
  ],
}
