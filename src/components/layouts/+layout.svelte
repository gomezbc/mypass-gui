<script lang="ts" context="module">
	import "../../app.css";
</script>

<script lang="ts">
	export let title: string;
</script>

<!DOCTYPE html>
<html lang="en">
	<head>
		<meta charset="UTF-8" />
		<meta name="description" content="Application for handling credentials with MongoDB" />
		<meta name="viewport" content="width=device-width" />
		<link rel="icon" type="image/svg+xml" href="/favicon.svg" />
		<meta name="generator" content={process.env.KIT} />
		<title>{title}</title>
	</head>
	<body class="relative dark:bg-slate-900 bg-gray-100 flex h-screen items-center py-16">
		<slot />
		<style>
			:root {
				color-scheme: light dark;
			}

			html {
				font-family: 'Onest Variable', system-ui, sans-serif;
				scroll-behavior: smooth;
			}

			body {
				background: #2a2a2a;
				display: flex;
				flex-direction: column;
				min-height: 100vh;
			}

			@media (prefers-reduced-motion: reduce) {
				html {
					scroll-behavior: auto;
				}
			}
		</style>
		<script src="https://cdnjs.cloudflare.com/ajax/libs/flowbite/1.8.0/flowbite.min.js"></script>
	</body>
</html>
