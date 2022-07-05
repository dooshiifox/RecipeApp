<script lang="ts">
	import type { BasicRecipe } from 'src/types/BasicRecipe';
	import { toRecipeString } from '$lib/../types/nutrient';
	import GradientBackground from '$lib/GradientBackground.svelte';
	import TimerIcon from '$lib/assets/icons/rounded/timer.svg?component';
	import UtensilsIcon from '$lib/assets/icons/rounded/utensils.svg?component';
	import BookmarkIcon from '$lib/assets/icons/rounded/bookmark.svg?component';
	import EmptyBookmarkIcon from '$lib/assets/icons/rounded/bookmark-border.svg?component';
	import Ratings from '$lib/recipe-view/Ratings.svelte';
	import { toTimeString } from '$lib/recipe-view/utils';

	export let recipe: BasicRecipe;

	console.log(recipe);
</script>

<GradientBackground gradient={recipe.gradient} class="flex w-full rounded-2xl">
	<img
		src={recipe.image}
		alt="Image of {recipe.title}"
		class="object-cover w-40 h-[120px] rounded-l-2xl rounded-r-3xl"
	/>
	<div class="grow flex flex-col py-2 px-4">
		<div class="flex">
			<div class="grow flex flex-col">
				<h1 class=" text-2xl font-bold text-black/80">{recipe.title}</h1>
				<p class="text-base font-bold text-black/50 pl-4">
					{toRecipeString(recipe.nutrients)}
				</p>
			</div>

			{#if recipe.bookmarked}
				<BookmarkIcon class="w-8 h-8 fill-black/60" />
			{:else}
				<EmptyBookmarkIcon class="w-8 h-8 fill-black/60" />
			{/if}
		</div>
		<!-- Fill any available vertical space so the info content
			is at the bottom of the result. -->
		<span class="grow" />
		<div class="flex justify-center w-full">
			<!-- Grid so that the Timer and Utensils have an equal size. -->
			<div class="grow grid grid-cols-2 justify-center">
				<!-- Time to make -->
				<div class="flex items-center gap-2">
					<TimerIcon class="w-5 h-5 fill-black/50" />
					<span class="text-lg text-black/60">{toTimeString(recipe.timeToCook)}</span>
				</div>
				<!-- Servings -->
				<div class="flex items-center gap-2">
					<UtensilsIcon class="w-5 h-5 fill-black/50" />
					<span class="text-lg text-black/60">Serves {recipe.servings}</span>
				</div>
			</div>
			<!-- Rating. If recipe.rating === undefined, still takes up the
				same amount of space, so things stay in line. -->
			<Ratings starClass="fill-black/60 w-6 h-6 -mr-1" stars={recipe.rating} />
		</div>
	</div>
</GradientBackground>
