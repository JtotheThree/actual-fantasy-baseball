module.exports = {
    mode: 'jit',
    content: [
        "./src/**/*.rs",
        "./src/index.html",
        "./src/**/*.html",
        "./src/**/*.css",
    ],
    theme: {
        extend: {
            colors: {
                'paper': '#f9f8f3',
            },
            fontFamily: {
                'pointedly': ['pointedly_madregular'],
                'blackmoon': ['blackmoon_questregular'],
                'blackmoon-italic': ['blackmoon_questitalic'],
                'railway': ['Raleway Webfont'],
                'title': ['germania_oneregular'],
                'normal': ['alegreyaregular']
            },
        }
    },
    variants: {
        display:['group-hover']
    },
};
