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
	import { BasicRecipe } from '$types/BasicRecipe';
	import { page } from '$app/stores';
	import type { APIErrorResponse } from '$utils/fetch';
	import { onMount } from 'svelte';
	import { getBookmarks, setBookmarks } from '$store/bookmarks';
	import { getRatings, setRatings } from '$store/ratings';

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
		// const nutrientFilters = e.detail.selectedFilters['Nutrients'] ?? [];
		// const lowercaseNutrients = new Set(
		// 	[...nutrientFilters].map((nutrient) => nutrient.toLowerCase())
		// );

		// const query = e.detail.query && new RegExp(e.detail.query.split('').join('.*'));
		// const resultsPerPage = e.detail.resultsPerPage;

		// searchResults = allRecipes
		// 	.filter((recipe) => {
		// 		if (lowercaseNutrients.size !== 0) {
		// 			// Iterate over every nutrient in the recipe,
		// 			// checking if *any* are in the nutrient filter.
		// 			// If so, the recipe is valid. If not, return false as it isn't.
		// 			if (!recipe.nutrients.some((nutrient) => lowercaseNutrients.has(nutrient.toLowerCase())))
		// 				return false;
		// 		}

		// 		if (query) {
		// 			// Check if the recipe name contains the query
		// 			if (!recipe.title.toLowerCase().match(query)) return false;
		// 		}

		// 		return true;
		// 	})
		// 	.slice(0, resultsPerPage);
		return [];
	}

	onMount(() => {
		let bookmarkSet = getBookmarks();
		if (bookmarkSet) {
			getAndAddRecipes(Array.from([...(bookmarkSet.values() || [])]), (recipe) => {
				savedRecipes.push(recipe);
			});
		}

		let ratings = getRatings();
		if (ratings) {
			// Sort the recipes by highest rated and get their ids.
			let sortedRatings = Object.entries(ratings)
				.sort((a, b) => b[1] - a[1])
				.map(([id, _]) => {
					return id;
				});

			getAndAddRecipes(sortedRatings || [], (recipe) => {
				savedRecipes.push(recipe);
			});
		}
	});

	function getAndAddRecipes(ids: string[], save: (recipe: BasicRecipe) => void) {
		// Gets the first 3 recipes from IDs.
		// If any recipe in that 3 encounters errors, the next recipe from
		// ids will be fetched, until either we have 3 recipes or we run out of ids.
		// The first 3 must be checked in parallel.
		// The output should be in the order given.

		const getSingle = () => {
			const id = ids.shift();
			if (id === undefined) return;

			BasicRecipe.getById(id).then(
				(recipe) => {
					save(recipe);
				},
				(e: APIErrorResponse<APIErrorResponse['error']>['error']) => {
					console.error(e);
					getSingle();
				}
			);
		};

		// Fetch the first 3
		for (let i = 0; i < 3; i++) {
			getSingle();
		}
	}
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="The Nutriblocks Recipe App" />
</svelte:head>

<section>
	<RecipeHeader recipeFn={BasicRecipe.getWeekly} />

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
