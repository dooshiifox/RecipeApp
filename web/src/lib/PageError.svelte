<script lang="ts">
	import { page } from '$app/stores';
	import GradientBackground from './utils/GradientBackground.svelte';

	let path = $page.url;
</script>

<div class="grid place-items-center w-full min-h-[70vh]">
	<GradientBackground
		starting="#ef8898"
		ending="#f9975e"
		class="w-[800px] mx-auto h-fit px-12 py-8 rounded-3xl flex flex-col gap-2 items-center justify-center"
	>
		<h1 class="text-4xl font-bold">
			<span class="text-black/80">There was an issue loading this page</span>
		</h1>
		<code class="bg-white/20 text-black/80 rounded px-4 py-0.5">{path}</code>
		<p class="text-black/80 text-xl leading-5 my-4">
			<slot>... although we don't quite know what the issue was...</slot>
		</p>
		<a
			href="/"
			class="px-8 py-3 bg-white/20 text-black/80 rounded-xl text-2xl font-bold hover:scale-[1.03] transition-transform"
		>
			Go to home page
		</a>
	</GradientBackground>
</div>
