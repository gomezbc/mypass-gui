<script lang="ts">
	import Layout from '../components/layouts/+layout.svelte';
	import CredentialSection from '../components/CredentialSection.svelte';
</script>

<Layout title="Home page">
	<main class="flex flex-col items-center justify-center h-screen w-screen lg:py-0">
		<div class="max-w-[85rem] w-full h-screen sm:px-6 lg:px-8 sm:py-6 lg:py-6 xl:py-8 mx-auto">
			<CredentialSection />
		</div>
	</main>
</Layout>