<script lang="ts">
	import type { Recipe } from '$types/Recipe';
	import Step from './Step.svelte';

	export let recipe: Recipe;
</script>

<div class="flex flex-col gap-16">
	{#each recipe.method.steps as step, i}
		<div class="{i % 2 ? 'pl-60' : 'pr-60'} w-full">
			<Step {step} index={i + 1} gradient={recipe.gradient} />
		</div>
	{/each}
</div>
