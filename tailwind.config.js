/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	prefix: 'tw-',
	theme: {
		extend: {}
	},
	corePlugins: {
		preflight: false
	},
	plugins: []
};
