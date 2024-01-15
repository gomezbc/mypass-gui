/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}', '!./src/tauri/**/*'],
	theme: {
		extend: {},
	},
	plugins: [],
	safelist: [/src-tauri/]
}
