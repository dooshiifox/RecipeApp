<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import RecipeHeader from '$lib/recipe-view/RecipeHeader.svelte';

	import ImageUrl from '../../static/images/omelette.jpg?url';
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
		imageUrl={ImageUrl}
		gradient={['#a6e8f4', '#a6b4f4']}
	/>
</section>
