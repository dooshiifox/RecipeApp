<script lang="ts">
	import { slide } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';

	import SearchIcon from '$lib/assets/icons/rounded/search.svg?component';
	import MoreIcon from '$lib/assets/icons/rounded/more.svg?component';
	import Filters from './Filters.svelte';
	import SearchResult from './SearchResult.svelte';
	import type { Filter } from './filters';
	import type { BasicRecipe } from 'src/types/BasicRecipe';

	export let query: string = '';

	export let filters: Filter[] = [
		{
			title: 'Nutrients',
			items: ['Protein', 'Carbohydrates', 'Iron', 'Fats', 'Vitamin A', 'Vitamin C']
		}
	];

	export let selectedFilters: Record<string, Set<string>> = {};

	export let searchResults: BasicRecipe[] = [];

	let showFilterBox = false;
	// Rotate clockwise each time the button is clicked
	let pressCount = 0;

	function showFilters(event: MouseEvent) {
		event.preventDefault();
		event.stopPropagation();
		showFilterBox = !showFilterBox;
		pressCount++;
	}
</script>

<div class="relative w-full">
	<div class="absolute inset-0 bg-[#2e2e30] shadow-lg -z-10" />
	<div class="py-8 mx-auto flex flex-col items-center">
		<!-- Search icon and text at top of container. -->
		<div class="flex gap-6 items-center">
			<SearchIcon class="w-16 h-16 fill-white/80" />
			<span class="text-[54px] font-bold text-white/90">Search</span>
		</div>
		<!-- Search box and filters -->
		<div class="mt-4 flex flex-col items-center">
			<div class="flex items-center gap-6 mb-4">
				<div
					class="rounded-xl h-12 w-full max-w-[540px] min-w-[360px] py-1 px-2 flex justify-center items-center bg-white border-black/5 border"
				>
					<SearchIcon class="ml-1 w-8 h-8 fill-black/30" />
					<input
						type="text"
						class="ml-3 w-full h-full text-2xl outline-none border-b border-b-transparent focus:border-b-black/50 transition-[border-bottom-color] duration-100"
						placeholder="Search for Recipes"
						bind:value={query}
					/>
				</div>
				<button class="p-1 rounded-xl bg-black/30" on:click={showFilters}>
					<MoreIcon
						class="fill-white/90 w-8 h-8 transition-[transform] duration-300"
						style={`transform: rotate(${pressCount * 180 + 180}deg);`}
					/>
				</button>
			</div>
			{#if showFilterBox}
				<!-- pb-4 to seperate the results from the filters only when its expanded. -->
				<div class="pb-4" transition:slide={{ delay: 0, duration: 300, easing: quintOut }}>
					<Filters {filters} bind:selectedFilters />
				</div>
			{/if}
		</div>
		<div class="w-[600px] flex flex-col items-center gap-4">
			<!-- Search results -->
			{#each searchResults as recipe}
				<SearchResult {recipe} />
			{:else}
				<div
					class="bg-white/10 rounded-4xl w-full py-6 text-3xl font-bold text-white/90 text-center"
				>
					No results found
				</div>
			{/each}
		</div>
	</div>
</div>
