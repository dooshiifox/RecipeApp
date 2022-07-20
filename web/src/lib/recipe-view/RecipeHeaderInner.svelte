<script lang="ts">
	import type { BasicRecipe } from '$types/BasicRecipe';
	import { toRecipeString } from '$types/nutrient';
	import { toTimeString } from './utils';

	import TimerIcon from '$icons/timer.svg?component';
	import UtensilsIcon from '$icons/utensils.svg?component';
	import Ratings from './Ratings.svelte';
	import GradientBackground from '../GradientBackground.svelte';
	import Bookmark from './Bookmark.svelte';

	export let recipe: BasicRecipe;
	export let hoverEffect = false;
</script>

<GradientBackground
	gradient={recipe.gradient}
	class="absolute inset-0 -z-10 skew-y-2 w-full h-full grid grid-cols-2"
>
	<img
		src={recipe.image}
		alt="Image of {recipe.title}"
		class="justify-self-end -skew-x-2 h-80 {hoverEffect
			? 'group-hover:h-[340px] transition-[height]'
			: ''} object-cover aspect-[7/4] rounded-3xl"
	/>
</GradientBackground>

<div class="w-full h-full grid grid-cols-2">
	<span />
	<div class="w-11/12 flex flex-row py-10 pl-8">
		<!-- Twists from upright and flat to the same angle as
			the image when hovered.
			Giving a width is required so a scrollbar does not appear.
			Put the hover effect here so it doesnt effect the bookmark icon.
		-->
		<div
			class="grow flex flex-col {hoverEffect
				? 'group-hover:pl-4 group-hover:pt-2 group-hover:skew-y-2 group-hover:-skew-x-2 transition-[padding-left,padding-top,transform]'
				: ''}"
		>
			{#if recipe.isWeekly}
				<span class="text-2xl font-bold text-black/40 leading-none">Weekly Recipe</span>
			{/if}
			<span class="text-4xl font-bold text-black/70 pb-1">{recipe.title}</span>
			<span class="pl-5 text-[28px] font-bold text-black/30">
				{toRecipeString(recipe.nutrients)}
			</span>
			<div class="flex items-center gap-3 mt-4">
				<TimerIcon class="w-8 h-8 fill-black/50" />
				<span class="text-2xl text-black/60">{toTimeString(recipe.timeToCook)}</span>
			</div>
			<div class="flex items-center gap-3 mt-1">
				<UtensilsIcon class="w-8 h-8 fill-black/50" />
				<span class="text-2xl text-black/60">Serves {recipe.servings}</span>
			</div>
			<Ratings stars={recipe.rating} class="mt-4" starClass="fill-black/70 w-8 h-8 -mr-1" />
		</div>
		<Bookmark
			{recipe}
			invisWrapperClass="w-16 h-16 hover:cursor-pointer"
			wrapperClass="p-0.5 rounded-lg"
			class="w-10 h-10 fill-black/60"
		/>
	</div>
</div>
