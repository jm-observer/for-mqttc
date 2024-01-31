
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src-web/**/*.{html,js}"],
  theme: {
    extend: {
      height: {
        inherit: 'inherit',
      },
      gridTemplateColumns: {
        content: '1fr 2fr',
      },
    },
  },
  plugins: [require("daisyui")],
}

