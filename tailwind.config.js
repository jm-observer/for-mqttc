
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src-web/**/*.{html,js}"],
  theme: {
    extend: {
      height: {
        inherit: 'inherit',
        84: '21rem',
      },
      gridTemplateColumns: {
        content: '1fr 2fr',
      },
      width: {
        130: '32.5rem',
        18: '4.5rem'
      },
      borderWidth: {
        1: '1px'
      }
    },
  },
  plugins: [require("daisyui")],
}

