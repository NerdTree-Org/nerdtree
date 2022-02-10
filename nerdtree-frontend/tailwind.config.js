module.exports = {
  theme: {
    extend: {
      colors: {
        red: '#FF3232',
      },
    },
    fontFamily: {
      monospace: ['JetBrains Mono', 'monospace'],
      sans: ['Inter', 'sans-serif'],
      header: ['Poppins', 'sans-serif'],
    },
  },
  variants: {},
  plugins: [],
  purge: {
    content: [
      `components/**/*.{vue,js}`,
      `layouts/**/*.vue`,
      `pages/**/*.vue`,
      `plugins/**/*.{js,ts}`,
      `nuxt.config.{js,ts}`,
    ],
  },
}
