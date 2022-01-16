module.exports = {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
  ],
  theme: {
    extend: {
        colors: {
            'paper': '#f9f8f3',
        },
        fontFamily: {
          'title': ['germania_oneregular'],
          'sans': ['alegreyaregular']
      },
    }
  },
  variants: {
      display:['group-hover']
  },
  plugins: [],
}
