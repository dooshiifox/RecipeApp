<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import Level from '$lib/homepage/Level.svelte';

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

	<div class="w-[720px] my-12 mx-auto">
		<Level level={1} currentXp={0} nextXp={100} />
	</div>
</section>
