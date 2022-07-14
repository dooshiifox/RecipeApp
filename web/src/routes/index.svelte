<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import RecipeHeader from '$lib/recipe-view/RecipeHeader.svelte';
	import Level from '$lib/homepage/Level.svelte';
	import Search from '$lib/homepage/search/Search.svelte';
	import Saved from '$lib/homepage/bottom_panels/Saved.svelte';
	import Rated from '$lib/homepage/bottom_panels/Rated.svelte';
	import FoundNothing from '$lib/homepage/FoundNothing.svelte';

	import generateBasicRecipes from '$faked/BasicRecipe';
	import type { BasicRecipe } from '$types/BasicRecipe';
	import { page } from '$app/stores';
	import { get } from '$utils/fetch';
	import { onMount } from 'svelte';

	let weeklyRecipe: BasicRecipe | undefined;
	async function getWeeklyRecipe(): Promise<BasicRecipe> {
		// Return from stored if available.
		if (weeklyRecipe) return weeklyRecipe;

		return get<BasicRecipe>('/weekly')
			.catch((e) => {
				return Promise.reject(
					'Server returned an unexpected value when retrieving weekly: ' + e.toString()
				);
			})
			.then((resp) => {
				if (resp.success) {
					weeklyRecipe = resp.data;
					return weeklyRecipe;
				} else {
					return Promise.reject('Server could not retrieve weekly: ' + resp.error);
				}
			});
	}

	let allRecipes: BasicRecipe[] = [];
	let savedRecipes: BasicRecipe[] = [];
	let ratedRecipes: BasicRecipe[] = [];

	/** The query for the Search function. */
	// http://localhost:3000 => ""
	// http://localhost:3000/?search => ""
	// http://localhost:3000/?search=ok => "ok"
	// http://localhost:3000/?search=some search query => "some search query"
	let query = $page.url.searchParams.get('search') ?? '';

	/** The recipes that match the query. */
	let searchResults: BasicRecipe[] = [];
	function onSearch(
		e: CustomEvent<{
			query: string;
			selectedFilters: Record<string, Set<string>>;
			resultsPerPage: number;
		}>
	) {
		const nutrientFilters = e.detail.selectedFilters['Nutrients'] ?? [];
		const lowercaseNutrients = new Set(
			[...nutrientFilters].map((nutrient) => nutrient.toLowerCase())
		);

		const query = e.detail.query && new RegExp(e.detail.query.split('').join('.*'));
		const resultsPerPage = e.detail.resultsPerPage;

		searchResults = allRecipes
			.filter((recipe) => {
				if (lowercaseNutrients.size !== 0) {
					// Iterate over every nutrient in the recipe,
					// checking if *any* are in the nutrient filter.
					// If so, the recipe is valid. If not, return false as it isn't.
					if (!recipe.nutrients.some((nutrient) => lowercaseNutrients.has(nutrient.toLowerCase())))
						return false;
				}

				if (query) {
					// Check if the recipe name contains the query
					if (!recipe.title.toLowerCase().match(query)) return false;
				}

				return true;
			})
			.slice(0, resultsPerPage);
	}

	onMount(() => {
		allRecipes = generateBasicRecipes(40);
		savedRecipes = allRecipes.filter((recipe) => recipe.bookmarked);
		ratedRecipes = allRecipes
			.filter((recipe) => recipe.rating !== undefined)
			.sort((a, b) => (b.rating as number) - (a.rating as number));
	});
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="The Nutriblocks Recipe App" />
</svelte:head>

<section>
	<RecipeHeader recipeFn={getWeeklyRecipe} />

	<div class="w-[720px] my-12 mx-auto">
		<Level />
	</div>

	<div class="mb-10">
		<Search {searchResults} {query} on:search={onSearch} />
	</div>

	<div class="flex flex-row justify-center gap-[120px]">
		<div class="w-[480px]">
			<Saved recipes={savedRecipes.slice(0, 3)} />
		</div>
		<div class="w-[480px]">
			<Rated recipes={ratedRecipes.slice(0, 3)} />
		</div>
	</div>

	<div class="mt-16">
		<FoundNothing />
	</div>
</section>
