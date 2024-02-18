
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
    },
  },
  plugins: [require("daisyui")],
}

