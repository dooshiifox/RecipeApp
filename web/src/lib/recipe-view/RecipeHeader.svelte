<script lang="ts">
	import { fade } from 'svelte/transition';
	import { toRecipeString } from '$types/nutrient';

	import TimerIcon from '$icons/timer.svg?component';
	import UtensilsIcon from '$icons/utensils.svg?component';
	import { toTimeString } from './utils';
	import Ratings from './Ratings.svelte';
	import type { BasicRecipe } from '$types/BasicRecipe';
	import GradientBackground from '../GradientBackground.svelte';
	import { onMount } from 'svelte';

	export let recipeFn: () => Promise<BasicRecipe>;

	let recipe: BasicRecipe | null = null;
	let error = false;
	$: loading = recipe === null;

	onMount(() => {
		recipeFn().then(
			(r) => {
				recipe = r;
			},
			(e) => {
				console.error(e);
				error = true;
			}
		);
	});
</script>

<div class="relative w-full h-80 skew-y-2 shadow-lg">
	{#if !recipe}
		<div transition:fade={{ duration: 50 }}>
			<GradientBackground
				gradient={['#0002', '#0004']}
				class="absolute w-full h-80 grid place-items-center"
			>
				<p class="text-2xl text-black/60 -skew-y-2">
					{#if loading}
						Loading...
					{:else if error}
						Sorry, there was a problem while getting the recipe to display here.
					{:else}
						Well, something's gone terribly wrong loading this!
					{/if}
				</p>
			</GradientBackground>
		</div>
	{:else}
		<a
			transition:fade={{ duration: 50 }}
			class="group h-80 block hover:h-[340px] transition-[height]"
			href="/recipe/{recipe.short}"
		>
			<GradientBackground gradient={recipe.gradient} class="w-full h-full grid grid-cols-2">
				<img
					src={recipe.image}
					alt="Image of {recipe.title}"
					class="justify-self-end -skew-x-2 h-80 group-hover:h-[340px] transition-[height] object-cover aspect-[7/4] rounded-3xl"
				/>
				<!-- Twists from upright and flat to the same angle as
					the image when hovered. Also moves 16px right due to the
					padding shift.
					Giving a width is required so a scrollbar does not appear.
				-->
				<div
					class="self-center w-11/12 flex flex-col py-10 pl-8 group-hover:pl-12 -skew-y-2 group-hover:skew-y-0 group-hover:-skew-x-2 transition-[padding-left,transform]"
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
			</GradientBackground>
		</a>
	{/if}
</div>
