<script lang="ts">
	import StarIcon from '$lib/assets/icons/rounded/star.svg?component';
	import TimerIcon from '$lib/assets/icons/rounded/timer.svg?component';
	import Ratings from '$lib/recipe-view/Ratings.svelte';
	import { toTimeString } from '$lib/recipe-view/utils';
	import BottomPanel from './BottomPanel.svelte';

	let recipes: {
		title: string;
		image: string;
		url: string;
		timeToMake: number;
		rating: number;
	}[] = [];

	recipes.push({
		title: 'Pumpkin Fritters',
		image:
			'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/pumpkin-fritters_737_373_c1.png',
		url: 'https://www.heartfoundation.org.nz/wellbeing/healthy-recipes/pumpkin-fritters',
		timeToMake: 25,
		rating: 5
	});
	recipes.push({
		title: 'Fresh Tomato and Capsicum Pasta ',
		image:
			'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/tomato-capsicum-pasta-sauce_737_373_c1.jpg',
		url: 'https://www.heartfoundation.org.nz/wellbeing/healthy-recipes/fresh-tomato-and-capsicum-pasta-sauce',
		timeToMake: 40,
		rating: 3.5
	});
</script>

<BottomPanel
	foregroundGradient={['#f7bc84', '#ed7272']}
	backgroundGradient={['#b27943', '#b43939']}
>
	<StarIcon slot="icon" class="w-10 h-10 fill-black/60" />
	<span slot="title" class="font-bold text-3xl text-black/70">Your Best Rated</span>
	<p slot="desc" class="text-[22px] leading-[22px] text-black/60 text-center">
		<span class="italic font-bold">Want to cook something delicious!</span><br />
		Have a look at your favourite recipes, such as...
	</p>
	<div slot="recipes" class="flex flex-col gap-3">
		{#each recipes as recipe, i}
			<!-- Reverse order of image and content if even. -->
			<a
				href={recipe.url}
				class="flex {i % 2
					? 'flex-row-reverse'
					: 'flex-row'} rounded-2xl bg-white/10 hover:bg-black/10 group transition-colors duration-100"
			>
				<img
					class="rounded-2xl w-[120px] aspect-[3/2] object-cover group-hover:brightness-90 transition-[filter] duration-100"
					src={recipe.image}
					alt="Image of {recipe.title}"
				/>
				<div class="min-h-20 grow flex flex-col items-center gap-2 py-1.5 px-4">
					<div class="flex items-center grow w-full">
						<span class="text-xl italic font-bold text-black/60 leading-5">{recipe.title}</span>
					</div>
					<div class="flex justify-center w-full">
						<div class="grow flex items-center justify-cen gap-2">
							<TimerIcon class="w-5 h-5 fill-black/50" />
							<span class="text-lg text-black/60">{toTimeString(recipe.timeToMake)}</span>
						</div>
						<Ratings starClass="fill-black/60 w-6 h-6 -mr-1" stars={recipe.rating} />
					</div>
				</div>
			</a>
		{/each}
	</div>
	<a
		href="/rated"
		slot="more"
		class="w-60 h-12 bg-black/20 hover:bg-black/10 transition-colors duration-100 rounded-2xl flex items-center justify-center"
	>
		<span class="text-white/90 text-xl font-bold">View More</span>
	</a>
</BottomPanel>
