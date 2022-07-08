<script lang="ts">
	import type { Gradient } from '$types/gradient';
	import { toRecipeString, type Nutrient } from '$types/nutrient';

	import TimerIcon from '$icons/timer.svg?component';
	import UtensilsIcon from '$icons/utensils.svg?component';
	import { toTimeString } from './utils';
	import Ratings from './Ratings.svelte';
	import type { BasicRecipe } from '$types/BasicRecipe';
	import GradientBackground from '../GradientBackground.svelte';

	export let isWeekly: boolean = false;
	export let recipe: BasicRecipe;
</script>

<GradientBackground
	gradient={recipe.gradient}
	class="w-full h-80 skew-y-2 grid shadow-lg grid-cols-2 justify-items-end"
>
	<img
		src={recipe.image}
		alt="Image of {recipe.title}"
		class="-skew-x-2 h-80 object-cover aspect-[7/4] rounded-3xl"
	/>
	<div class="flex flex-col h-full pt-10 pl-8 justify-self-start -skew-y-2">
		{#if isWeekly}
			<span class="text-2xl font-bold text-black/40 leading-none">Weekly Recipe</span>
		{/if}
		<span class="text-4xl font-bold text-black/70 pb-1">{recipe.title}</span>
		<span class="pl-5 text-[28px] font-bold text-black/30">{toRecipeString(recipe.nutrients)}</span>
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
</GradientBackground>
