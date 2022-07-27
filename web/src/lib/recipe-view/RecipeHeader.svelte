<script lang="ts">
	import { fade } from 'svelte/transition';

	import type { BasicRecipe } from '$types/BasicRecipe';
	import GradientBackground from '$lib/utils/GradientBackground.svelte';
	import { onMount } from 'svelte';
	import RecipeHeaderInner from './RecipeHeaderInner.svelte';

	export let recipeFn: () => Promise<BasicRecipe>;
	export let hoverEffect = false;

	let recipe: BasicRecipe | null = null;
	let error = false;
	$: loading = recipe === null;

	onMount(() => {
		recipeFn().then(
			(r) => {
				recipe = r;
			},
			(e) => {
				console.error(e);
				error = true;
			}
		);
	});
</script>

<div class="relative w-full h-80">
	{#if !recipe}
		<div transition:fade={{ duration: 50 }}>
			<GradientBackground
				gradient={['#0002', '#0004']}
				class="absolute inset-0 skew-y-2 shadow-lg"
			/>
			<div class="w-full h-80 grid place-items-center">
				<p class="text-2xl text-black/60 -skew-y-2">
					{#if error}
						Sorry, there was a problem while getting the recipe to display here.
					{:else if loading}
						Loading...
					{:else}
						Well, something's gone terribly wrong loading this!
					{/if}
				</p>
			</div>
		</div>
	{:else if hoverEffect}
		<a
			transition:fade={{ duration: 50 }}
			class="relative h-80 block group hover:h-[340px] transition-[height]"
			href="/recipe/{recipe.short}"
		>
			<RecipeHeaderInner {recipe} {hoverEffect} />
		</a>
	{:else}
		<div transition:fade={{ duration: 50 }} class="relative h-80 block">
			<RecipeHeaderInner {recipe} {hoverEffect} />
		</div>
	{/if}
</div>
