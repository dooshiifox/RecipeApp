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

	let allRecipes: BasicRecipe[] = generateBasicRecipes(200);
	console.debug('Recipes generated.');

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
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="The Nutriblocks Recipe App" />
</svelte:head>

<section>
	<RecipeHeader
		isWeekly
		title="Home-made Omelette"
		nutrients={['Vitamin A', 'Iron']}
		timeToMake={15}
		servings={1}
		imageUrl="/images/omelette.jpg"
		gradient={['#a6e8f4', '#a6b4f4']}
	/>

	<div class="w-[720px] my-12 mx-auto">
		<Level />
	</div>

	<div class="mb-10">
		<Search {searchResults} on:search={onSearch} />
	</div>

	<div class="flex flex-row items-center justify-center gap-[120px]">
		<div class="w-[480px]">
			<Saved />
		</div>
		<div class="w-[480px]">
			<Rated />
		</div>
	</div>

	<div class="mt-16 -z-20 relative">
		<FoundNothing />
	</div>
</section>
