<script lang="ts">
	import type { Recipe } from '$types/Recipe';
	import ListIcon from '$icons/list.svg?component';
	import Checkbox from './Checkbox.svelte';
	import Basket from './Basket.svelte';

	export let recipe: Recipe;
</script>

<div class="rounded-3xl bg-black/10 w-full px-6 py-4">
	<div class="flex flex-row gap-4 items-center justify-center">
		<ListIcon class="w-8 h-8" />
		<span class="text-3xl font-bold italic">Ingredients</span>
	</div>
	<!-- Gap is 16px horizontally, 8px vertically -->
	<div
		class="mt-4 grid gap-[8px_16px] w-full auto-rows-auto grid-cols-[min-content,1fr,min-content] items-center"
	>
		{#each recipe.ingredients as ingredient}
			<Checkbox
				class="w-9 h-9 p-1 cursor-pointer hover:bg-black/10 [.active&]:fill-green-600 [.inactive&]:fill-black transition-[background-color] duration-75 rounded-lg"
			/>
			<span class="text-2xl italic leading-5">{ingredient}</span>
			<Basket
				class="w-9 h-9 p-1 cursor-pointer hover:bg-black/10 fill-black transition-[background-color] duration-75 rounded-lg"
			/>
		{/each}
	</div>
</div>
