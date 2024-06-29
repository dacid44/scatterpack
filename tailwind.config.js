/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	prefix: 'tw-',
	theme: {
		extend: {
			colors: {
				primary: 'var(--bs-primary)',
				secondary: 'var(--bs-secondary)',
				success: 'var(--bs-success)',
				info: 'var(--bs-info)',
				warning: 'var(--bs-warning)',
				danger: 'var(--bs-danger)',
				light: 'var(--bs-light)',
				dark: 'var(--bs-dark)',
			}
		}
	},
	important: '#app-body',
	corePlugins: {
		preflight: false
	},
	plugins: []
};
