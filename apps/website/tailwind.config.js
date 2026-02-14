/** @type {import('tailwindcss').Config} */

module.exports = {
  content: {
    files: ['*.html', './src/**/*.rs'],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, ' ')
    }
  },
  darkMode: 'selector',
  theme: {
    extend: {
      fontFamily: {
        "opensans": ['Open Sans', 'sans-serif'],
        "poppins": ['Poppins', 'sans-serif'],
        "mono": ['Roboto Mono', 'monospace'],
      },
      boxShadow: {
        'smoke-shadow': '0 8px 26px 0 rgba(22,24,26,0.15)',
      },
      colors: {
        'white': '#ffffff',
        'smoke': '#f2f4f7',
        'deepsea': '#095256',
        'teal': '#087F8C',
        'zomp': '#5AAA95',
        'asparagus': '#86a873',
        'sheengold': '#bb9f06',
        'zeus': '#242424',
      },
      transitionTimingFunction: {
        'custom': 'cubic-bezier(0.165, 0.84, 0.44, 1)',
      },
      keyframes: {
        lineMove: {
          '0%': {
            top: '-80px',
          },
          '100%': {
            top: '100%',
          }
        },
        loaderLetter: {
          '0%': {
            opacity: '1',
          },
          '100%': {
            opacity: '0',
          }
        },
      },
      animation: {
        bgLine: 'lineMove 8s linear infinite',
        loader: 'loaderLetter 1.0s linear infinite',
      },
    },
  },
  plugins: [],
}