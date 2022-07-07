<script lang="ts">
	import GradientBackground from '$lib/GradientBackground.svelte';
	import type { Gradient } from '$types/gradient';

	export let foregroundGradient: Gradient;
	export let backgroundGradient: Gradient;
	export let isEmpty: boolean;
</script>

<div class="relative">
	<GradientBackground class="w-full rounded-4xl flex flex-col" gradient={foregroundGradient}>
		<div class="pt-4 flex flex-row gap-4 justify-center items-center">
			<slot name="icon" />
			<slot name="title" />
		</div>
		{#if isEmpty}
			<div class="w-3/4 py-4 mx-auto">
				<slot name="empty-desc" />
			</div>
		{:else}
			<div class="w-3/4 pt-4 mx-auto">
				<slot name="desc" />
			</div>
			<div class="w-full px-8 my-4">
				<slot name="recipes" />
			</div>
			<div class="py-4 mx-auto">
				<slot name="more" />
			</div>
		{/if}
	</GradientBackground>
	<GradientBackground
		class="absolute inset-0 translate-x-3 translate-y-3 rounded-4xl opacity-30 -z-10"
		gradient={backgroundGradient}
	/>
</div>
