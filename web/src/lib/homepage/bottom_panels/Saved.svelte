<script lang="ts">
	import BookmarkIcon from '$lib/assets/icons/rounded/bookmark.svg?component';
	import TimerIcon from '$lib/assets/icons/rounded/timer.svg?component';
	import UtensilsIcon from '$lib/assets/icons/rounded/utensils.svg?component';
	import { toTimeString } from '$lib/recipe-view/utils';
	import BottomPanel from './BottomPanel.svelte';

	let recipes: {
		title: string;
		image: string;
		url: string;
		timeToMake: number;
		servings: number;
	}[] = [];

	recipes.push({
		title: 'Pumpkin Fritters',
		image:
			'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/pumpkin-fritters_737_373_c1.png',
		url: 'https://www.heartfoundation.org.nz/wellbeing/healthy-recipes/pumpkin-fritters',
		timeToMake: 25,
		servings: 10
	});
	recipes.push({
		title: 'Fresh Tomato and Capsicum Pasta ',
		image:
			'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/tomato-capsicum-pasta-sauce_737_373_c1.jpg',
		url: 'https://www.heartfoundation.org.nz/wellbeing/healthy-recipes/fresh-tomato-and-capsicum-pasta-sauce',
		timeToMake: 40,
		servings: 8
	});
</script>

<BottomPanel
	foregroundGradient={['#f784d6', '#cf67ef']}
	backgroundGradient={['#bc479a', '#9633b4']}
>
	<BookmarkIcon slot="icon" class="w-10 h-10 fill-black/60" />
	<span slot="title" class="font-bold text-3xl text-black/70">Your Saved Recipes</span>
	<p slot="desc" class="text-[22px] leading-[22px] text-black/60 text-center">
		<span class="italic font-bold">These look pretty tasty!</span><br />
		Go through all those recipes you were saving for later, such as...
	</p>
	<div slot="recipes" class="flex flex-col gap-3">
		{#each recipes as recipe, i}
			<!-- Reverse order of image and content if even. -->
			<a
				href={recipe.url}
				class="flex {i % 2
					? 'flex-row-reverse'
					: 'flex-row'} rounded-2xl bg-white/10 hover:bg-black/10 group"
			>
				<img
					class="rounded-2xl w-[120px] aspect-[3/2] object-cover group-hover:brightness-90"
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
							<span class="text-lg text-black/60">{toTimeString(recipe.timeToMake)}</span>
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
		class="w-60 h-12 bg-[#9c3db9] hover:bg-[#ac58c5] rounded-2xl flex items-center justify-center"
	>
		<span class="text-white/90 text-xl font-bold">View More</span>
	</a>
</BottomPanel>
