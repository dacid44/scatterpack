import flowbitePlugin from 'flowbite/plugin';

/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				'bs-primary': 'var(--bs-primary)',
				'bs-secondary': 'var(--bs-secondary)',
				'bs-success': 'var(--bs-success)',
				'bs-info': 'var(--bs-info)',
				'bs-warning': 'var(--bs-warning)',
				'bs-danger': 'var(--bs-danger)',
				'bs-light': 'var(--bs-light)',
				'bs-dark': 'var(--bs-dark)',
				// flowbite-svelte
				primary: {
					50: '#FFF5F2',
					100: '#FFF1EE',
					200: '#FFE4DE',
					300: '#FFD5CC',
					400: '#FFBCAD',
					500: '#FE795D',
					600: '#EF562F',
					700: '#EB4F27',
					800: '#CC4522',
					900: '#A5371B'
				}
			}
		}
	},
	important: '#app-body',
	corePlugins: {
		// preflight: false
	},
	plugins: [flowbitePlugin]
};
