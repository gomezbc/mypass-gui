/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}',
		'./node_modules/flowbite/**/*.js',
		'./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}',
		'!./src-tauri/target/*.*'],
	theme: {
		extend: {},
	},
	plugins: [require('flowbite/plugin')]
}
