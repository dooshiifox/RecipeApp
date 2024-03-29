<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import { slide } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';

	import SearchIcon from '$icons/search.svg?component';
	import MoreIcon from '$icons/more.svg?component';
	import Filters from './Filters.svelte';
	import SearchResult from './SearchResult.svelte';
	import type { Filter } from './filters';
	import type { BasicRecipe } from '$types/BasicRecipe';
	import { Cooldown } from '$src/utils/cooldown';

	const dispatch = createEventDispatcher();

	/** Dispatches a search event on the component. */
	function dispatchSearch() {
		dispatch('search', {
			query,
			selectedFilters,
			resultsPerPage
		});
	}

	/** A cooldown for the onChange method. */
	const cooldown = new Cooldown(500, onChange);

	/** A list of all available filters on the query. */
	export let filters: Filter[] = [
		{
			title: 'Macronutrients',
			items: [
				'Protein',
				'Carbohydrates',
				'Fiber',
				'Fat',
				'Saturated Fat',
				'Cholesterol',
				'Calories',
				'Sugar'
			]
		},
		{
			title: 'Minerals',
			items: [
				'Iron',
				'Calcium',
				'Magnesium',
				'Zinc',
				'Copper',
				'Iodine',
				'Manganese',
				'Phosphorus',
				'Potassium',
				'Selenium',
				'Sodium'
			]
		},
		{
			title: 'Vitamins',
			items: [
				'Vitamin A',
				'Vitamin B1',
				'Vitamin B2',
				'Vitamin B3',
				'Vitamin B5',
				'Vitamin B6',
				'Vitamin B9',
				'Vitamin B12',
				'Vitamin C',
				'Vitamin D',
				'Vitamin E',
				'Vitamin K',
				'Choline'
			]
		}
	];

	/** The query string the user is searching for. */
	export let query: string = '';
	/** All selected filters the user is searching for.
	 *
	 * Key of the record is the title of the filter, and each element in the set
	 * is an option from the filter to apply.
	 */
	export let selectedFilters: Record<string, Set<string>> = {};

	/** All the search results to display. */
	export let searchResults: BasicRecipe[] = [];
	/** The number of results to load. */
	// Initially 3 unless the `query` is set by the caller component.
	export let resultsPerPage: number = query === '' ? 3 : 10;

	function onChange() {
		// If query or selectedFilters change, set the number of results
		// per page to 10 from 3.
		resultsPerPage = 10;
		// Dispatch an event.
		dispatchSearch();
	}

	// Whether to show the filters or not.
	let showFilterBox = false;
	// Rotate filter dropdown button clockwise each time the button is clicked
	let pressCount = 0;

	onMount(() => {
		// Dispatch a search immediately upon loading the component.
		dispatchSearch();
	});
</script>

<div class="relative w-full">
	<div class="absolute inset-0 bg-[#2e2e30] -z-10 skew-y-2 background-shadow" />
	<div class="py-8 mx-auto flex flex-col items-center">
		<!-- Search icon and text at top of container. -->
		<div class="flex gap-6 items-center">
			<SearchIcon class="w-16 h-16 fill-white/80" />
			<span class="text-[54px] font-bold text-white/90">Search</span>
		</div>
		<!-- Search box and filters -->
		<div class="mt-6 flex flex-col items-center">
			<!-- Search box wrapper -->
			<div class="flex items-center gap-6 mb-4">
				<!-- Search box -->
				<div
					class="rounded-xl h-12 w-[420px] py-1 pl-2 pr-4 flex justify-center items-center bg-white border-black/5 border"
				>
					<SearchIcon class="ml-1 w-8 h-8 fill-black/30" />
					<input
						type="text"
						class="ml-3 grow h-full text-2xl outline-none border-b border-b-transparent focus:border-b-black/50 transition-[border-bottom-color] duration-100"
						placeholder="Search for Recipes"
						bind:value={query}
						on:input={() => cooldown.use()}
					/>
				</div>
				<!-- Open/Close filters button -->
				<button
					class="p-1 rounded-xl bg-black/30 w-max h-max"
					on:click|preventDefault|stopPropagation={() => {
						showFilterBox = !showFilterBox;
						pressCount++;
					}}
				>
					<MoreIcon
						class="fill-white/90 w-8 h-8 transition-[transform] duration-300"
						style={`transform: rotate(${pressCount * 180 + 180}deg);`}
					/>
				</button>
			</div>
			{#if showFilterBox}
				<!-- pb-4 to seperate the results from the filters only when its expanded. -->
				<div class="pb-4" transition:slide={{ delay: 0, duration: 300, easing: quintOut }}>
					<Filters {filters} bind:selectedFilters on:change={() => cooldown.use()} />
				</div>
			{/if}
		</div>
		<div class="w-[600px] space-y-4 mt-4">
			<!-- Search results -->
			{#each searchResults as recipe (recipe.id)}
				<div>
					<SearchResult {recipe} />
				</div>
			{:else}
				<div
					class="bg-white/10 rounded-4xl w-full py-6 text-3xl font-bold text-white/90 text-center"
				>
					No results found
				</div>
			{/each}
		</div>
		<!-- Load more button -->
		{#if resultsPerPage === 3}
			<div class="flex flex-col items-center space-y-4 mt-4">
				<button
					class="bg-white/10 hover:bg-white/20 transition-colors rounded-2xl w-full px-12 py-4 text-2xl font-bold text-white/90 text-center"
					on:click={() => {
						resultsPerPage = 10;
						dispatchSearch();
					}}
				>
					Load more results
				</button>
			</div>
		{/if}
	</div>
</div>

<style>
	.background-shadow {
		/* Box shadow on top and bottom */
		box-shadow: inset 0 8px 8px -8px #fff8, inset 0 -8px 8px -8px #fff8;
	}
</style>
