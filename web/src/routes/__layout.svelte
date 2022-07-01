<script lang="ts">
	import Header from '$lib/header/Header.svelte';
	import Footer from '$lib/footer/Footer.svelte';
	import '../app.css';
</script>

<div class="flex flex-col min-h-screen">
	<Header />

	<main class="grow">
		<slot />
	</main>

	<Footer />
</div>
