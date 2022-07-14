<script lang="ts">
	import BottomPanel from './BottomPanel.svelte';
	import BookmarkIcon from '$icons/bookmark.svg?component';
	import TimerIcon from '$icons/timer.svg?component';
	import UtensilsIcon from '$icons/utensils.svg?component';
	import type { BasicRecipe } from '$types/BasicRecipe';
	import { toTimeString } from '$lib/recipe-view/utils';

	export let recipes: BasicRecipe[] = [];
</script>

<BottomPanel
	foregroundGradient={['#f784d6', '#cf67ef']}
	backgroundGradient={['#bc479a', '#9633b4']}
	isEmpty={recipes.length === 0}
>
	<BookmarkIcon slot="icon" class="w-10 h-10 fill-black/60" />
	<span slot="title" class="font-bold text-3xl text-black/70">Your Saved Recipes</span>
	<div slot="empty-desc" class="text-[22px] leading-[22px] text-black/60 text-center">
		<p class="italic font-bold">Hang on, you don't have any saved recipes?!</p>
		<br />
		<p>
			You can click that <BookmarkIcon class="inline fill-black/70" /> Bookmark icon to save your recipes.
		</p>
	</div>
	<p slot="desc" class="text-[22px] leading-[22px] text-black/60 text-center">
		<span class="italic font-bold">These look pretty tasty!</span><br />
		Go through all those recipes you were saving for later, such as...
	</p>
	<div slot="recipes" class="flex flex-col gap-3">
		{#each recipes as recipe, i}
			<!-- Reverse order of image and content if even. -->
			<a
				href={recipe.short}
				class="flex {i % 2
					? 'flex-row-reverse'
					: 'flex-row'} rounded-2xl bg-white/10 hover:bg-black/10 group transition-colors duration-100"
			>
				<img
					class="rounded-2xl w-[120px] aspect-[3/2] object-cover group-hover:brightness-90 transition-[filter] duration-100"
					src={recipe.image}
					alt="Image of {recipe.title}"
				/>
				<div class="min-h-20 grow flex flex-col items-center gap-2 py-2 px-4">
					<div class="flex items-center grow w-full">
						<span class="text-xl italic font-bold text-black/60 leading-5">{recipe.title}</span>
					</div>
					<div class="flex justify-center w-full">
						<div class="grow flex items-center gap-2">
							<TimerIcon class="w-5 h-5 fill-black/50" />
							<span class="text-lg text-black/60">{toTimeString(recipe.timeToCook)}</span>
						</div>
						<div class="grow flex items-center gap-2">
							<UtensilsIcon class="w-5 h-5 fill-black/50" />
							<span class="text-lg text-black/60">Serves {recipe.servings}</span>
						</div>
					</div>
				</div>
			</a>
		{/each}
	</div>
	<a
		href="/saved"
		slot="more"
		class="w-60 h-12 bg-black/20 hover:bg-black/10 transition-colors duration-100 rounded-2xl flex items-center justify-center"
	>
		<span class="text-white/90 text-xl font-bold">View More</span>
	</a>
</BottomPanel>
