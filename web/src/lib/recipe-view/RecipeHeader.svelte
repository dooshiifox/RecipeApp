<script lang="ts">
	import type { Gradient } from '../../types/gradient';
	import { toRecipeString, type Nutrient } from '../../types/nutrient';

	import TimerIcon from '$lib/assets/icons/rounded/timer.svg?component';
	import UtensilsIcon from '$lib/assets/icons/rounded/utensils.svg?component';
	import { toTimeString } from './utils';
	import Ratings from './Ratings.svelte';

	export let isWeekly: boolean = false;
	export let title: string;
	export let nutrients: Nutrient[];
	export let timeToMake: number;
	export let servings: number;
	export let imageUrl: string;
	export let gradient: Gradient;
</script>

<div class="relative w-full h-80 -z-10">
	<div
		class="absolute inset-0 skew-y-2 flex justify-center bg-gradient-to-br from-[var(--gradient-1)] to-[var(--gradient-2)] shadow-lg"
		style="--gradient-1:{gradient[0]}; --gradient-2:{gradient[1]}"
	>
		<!-- combination of justify-center and -translate-x-1/2 means this is
				placed with the rightmost px at the center of the screen -->
		<img
			src={imageUrl}
			alt={title}
			class="-skew-x-2 h-full object-cover aspect-[7/4] rounded-3xl -translate-x-1/2"
		/>
	</div>
	<div class="flex justify-center">
		<div class="flex flex-col translate-x-1/2 h-full pt-10 pl-8">
			{#if isWeekly}
				<span class="text-2xl font-bold text-black/40 leading-none">Weekly Recipe</span>
			{/if}
			<span class="text-4xl font-bold text-black/70 pb-1">{title}</span>
			<span class="pl-5 text-[28px] font-bold text-black/30">{toRecipeString(nutrients)}</span>
			<div class="flex items-center gap-3 mt-4">
				<TimerIcon class="w-8 h-8 fill-black/50" />
				<span class="text-2xl text-black/60">{toTimeString(timeToMake)}</span>
			</div>
			<div class="flex items-center gap-3 mt-1">
				<UtensilsIcon class="w-8 h-8 fill-black/50" />
				<span class="text-2xl text-black/60">Serves {servings}</span>
			</div>
			<Ratings stars={3.5} class="mt-4" starClass="fill-black/70 w-8 h-8 -mr-1" />
		</div>
	</div>
</div>
