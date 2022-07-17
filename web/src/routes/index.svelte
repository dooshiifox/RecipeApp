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

	import { BasicRecipe, type BasicRecipeConstructorUuid } from '$types/BasicRecipe';
	import { page } from '$app/stores';
	import { post, type APIErrorResponse } from '$utils/fetch';
	import { onMount } from 'svelte';
	import { getBookmarks } from '$store/bookmarks';
	import { getRatings } from '$store/ratings';

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
	/** Used for measuring whether this is the current reqeust.
	 * Copy it into a variable before the request begins, and check after we
	 * get a result and increment it. If they don't match, there has been
	 * a request sent in the meantime which has had a result, meaning we
	 * do not update again.
	 *
	 * https://cdn.discordapp.com/attachments/272233059527557131/998044508187009034/unknown.png
	 */
	let searchCount = 0;

	async function onSearch(
		e: CustomEvent<{
			query: string;
			selectedFilters: Record<string, Set<string>>;
			resultsPerPage: number;
		}>
	) {
		console.debug('Searching for recipes...');

		searchCount++;
		let searchCountCheck = searchCount;

		// Generate the request body contents.
		const reqBody = {} as Record<string, any>;

		const getFilter = (name: string) => e.detail.selectedFilters[name] ?? new Set();
		const nutrients = [
			...getFilter('Macronutrients'),
			...getFilter('Minerals'),
			...getFilter('Vitamins')
		];

		if (nutrients.length > 0) {
			reqBody['nutrients'] = nutrients;
		}
		if (e.detail.query.length !== 0) {
			reqBody['query'] = e.detail.query;
		}
		reqBody['resultsPerPage'] = e.detail.resultsPerPage;

		// Send the request.
		const localSearchResults = await post<BasicRecipeConstructorUuid[]>(
			'/search',
			JSON.stringify(reqBody)
		).then(
			(res) => {
				if (res.success) {
					return res.data.map((r) => new BasicRecipe(r));
				} else {
					console.error(`API error fetching search: ${JSON.stringify(res.error)}`);
					return [];
				}
			},
			(e: APIErrorResponse) => {
				console.error(`Unexpected error fetching search: ${JSON.stringify(e.error)}`);
				return [];
			}
		);

		// Check the request is still valid.
		// Check `searchCount` docs for more on what this means.
		if (searchCountCheck === searchCount) {
			searchResults = localSearchResults;
		} else {
			console.debug('Search request was cancelled.');
		}
	}

	onMount(() => {
		let bookmarkSet = getBookmarks();
		if (bookmarkSet) {
			getAndAddRecipes([...(bookmarkSet.values() || [])], (recipe) => {
				savedRecipes.push(recipe);
				savedRecipes = savedRecipes; // Re-render components
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
				ratedRecipes.push(recipe);
				ratedRecipes = ratedRecipes; // Re-render components
			});
		}
	});

	function getAndAddRecipes(ids: string[], save: (recipe: BasicRecipe) => void) {
		// Gets the first 3 recipes from IDs.
		// If any recipe in that 3 encounters errors, the next recipe from
		// ids will be fetched, until either we have 3 recipes or we run out of ids.
		// The first 3 must be checked in parallel.
		// The output should be in the order given.
		let getterCount = 0;

		const getSingle = () => {
			const id = ids.shift();
			if (id === undefined) return;
			getterCount++;
			if (getterCount > 10) {
				console.error('Too many recipes attempted to be fetched.');
				return;
			}

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
	<RecipeHeader recipeFn={BasicRecipe.getWeekly} hoverEffect={true} />

	<div class="w-[720px] my-12 mx-auto">
		<Level />
	</div>

	<div class="mb-10">
		<Search {searchResults} {query} on:search={onSearch} />
	</div>

	<div class="flex flex-row justify-center gap-[120px]">
		<div class="w-[480px]">
			<Saved recipes={savedRecipes} />
		</div>
		<div class="w-[480px]">
			<Rated recipes={ratedRecipes} />
		</div>
	</div>

	<div class="mt-16">
		<FoundNothing />
	</div>
</section>
