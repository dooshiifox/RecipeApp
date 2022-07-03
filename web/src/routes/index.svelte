<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import RecipeHeader from '$lib/recipe-view/RecipeHeader.svelte';
	import Level from '$lib/homepage/Level.svelte';
	import Saved from '$lib/homepage/bottom_panels/Saved.svelte';
	import Rated from '$lib/homepage/bottom_panels/Rated.svelte';
	import FoundNothing from '$lib/homepage/FoundNothing.svelte';

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
		<Level />
	</div>
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
