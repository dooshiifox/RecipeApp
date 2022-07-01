const colors = require('tailwindcss/colors');
const defaultTheme = require('tailwindcss/defaultTheme');

// Colours that don't cause Tailwind to complain about old colour names
let warnlessColors = {};
for (let color in colors) {
	if (!['lightBlue', 'warmGray', 'trueGray', 'coolGray', 'blueGray'].includes(color)) {
		warnlessColors[color] = colors[color];
	}
}

/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	// We should be able to manually toggle it.
	darkMode: 'class',
	theme: {
		fontFamily: {
			sans: ['Louis George Cafe', ...defaultTheme.fontFamily.sans],
			serif: ['Merriweather', 'serif'],
			mono: ['droid-sans-mono-slashed', ...defaultTheme.fontFamily.mono]
		},
		colors: {
			...warnlessColors
		},
		extend: {
			fontSize: {
				'2.5xl': [
					'28px',
					{
						letterSpacing: '-0.02em',
						lineHeight: '32px'
					}
				]
			}
		}
	},
	plugins: []
};
