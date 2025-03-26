/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['./src/**/*.{html,js,ts,tsx}'],
    theme: {
      extend: {
        keyframes: {
          'gradient-x': {
            '0%, 100%': {
              'background-position': '0% 50%',
            },
            '50%': {
              'background-position': '100% 50%',
            },
          },
        },
        animation: {
          'gradient-x': 'gradient-x 8s ease infinite',
        },
      },
    },
    plugins: [],
    corePlugins: {
      preflight: false,
    },
  }