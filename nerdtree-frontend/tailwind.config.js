module.exports = {
  theme: {
    extend: {
      colors: {
        red: "#FF3232",
        gray: "#1f1f1f"
      },
    },
    fontFamily: {
      monospace: ['JetBrains Mono', 'monospace'],
      sans: ['Inter', 'sans-serif']
    }
  },
  variants: {},
  plugins: [],
  purge: {
    content: [
      `components/**/*.{vue,js}`,
      `layouts/**/*.vue`,
      `pages/**/*.vue`,
      `plugins/**/*.{js,ts}`,
      `nuxt.config.{js,ts}`
    ]
  }
}
