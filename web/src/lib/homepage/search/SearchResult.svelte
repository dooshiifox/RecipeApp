<script lang="ts">
	import type { BasicRecipe } from '$types/BasicRecipe';
	import { toRecipeString } from '$types/nutrient';
	import GradientBackground from '$lib/utils/GradientBackground.svelte';
	import TimerIcon from '$icons/timer.svg?component';
	import UtensilsIcon from '$icons/utensils.svg?component';
	import Ratings from '$lib/recipe-view/Ratings.svelte';
	import { toTimeString } from '$lib/recipe-view/utils';
	import { browser } from '$app/env';
	import Bookmark from '$src/lib/recipe-view/Bookmark.svelte';

	export let recipe: BasicRecipe;
</script>

<a
	class="block group w-full h-[120px] hover:scale-[1.03] transition-transform"
	href="/recipe/{recipe.short}"
>
	<GradientBackground
		gradient={recipe.gradient}
		class="flex h-full w-full rounded-2xl group-hover:brightness-105 transition-[filter]"
	>
		<img
			src={recipe.image}
			alt="Image of {recipe.title}"
			class="object-cover aspect-[4/3] rounded-l-2xl rounded-r-3xl"
		/>
		<div class="grow flex flex-col">
			<!-- Title, nutrients, and bookmark -->
			<div class="flex gap-2">
				<div class="grow flex flex-col mt-2 ml-4">
					<h1 class=" text-2xl font-bold text-black/80 leading-6">{recipe.title}</h1>
					<p class="text-base font-bold text-black/50 pl-4 mt-0.5">
						{toRecipeString(recipe.nutrients)}
					</p>
				</div>

				<Bookmark
					{recipe}
					invisWrapperClass="w-12 h-12"
					wrapperClass="p-1 rounded-lg"
					class="w-8 h-8 fill-black/60"
				/>
			</div>
			<!-- Fill any available vertical space so the info content
			is at the bottom of the result. -->
			<span class="grow" />
			<div class="flex justify-center w-full pb-2 px-4">
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
				{#if browser}
					<Ratings starClass="fill-black/60 w-6 h-6 -mr-1" stars={recipe.rating} />
				{:else}
					<Ratings starClass="fill-black/60 w-6 h-6 -mr-1" stars={undefined} />
				{/if}
			</div>
		</div>
	</GradientBackground>
</a>
