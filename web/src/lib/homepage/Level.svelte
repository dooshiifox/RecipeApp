<script lang="ts" context="module">
	export interface Animation {
		fromXp: number;
		toXp: number;
	}
</script>

<script lang="ts">
	import TrophyIcon from '$icons/trophy.svg?component';
	import { Confetti } from 'svelte-confetti';
	import { browser } from '$app/env';
	import { levelingInfo, getLevel, type LevelingInfo } from '$store/level';
	import EASING from '$utils/easing';

	export let showQuote = true;
	export let animate: false | Animation = false;

	/** A collection of quotes that can be displayed below the
	 * progress bar.
	 */
	const quotes = [
		'Each day a little closer!',
		"Take it slow, and it'll grow.",
		'Just a little more!',
		'If you keep going it will too!',
		'Gaining experience is as simple as answering quizzes!',
		'TIP: You get more experience by doing the weekly recipe!',
		"TIP: By getting all questions correct, you'll get bonus experience!"
	];

	const LOADING = -1;

	/** The user's current level. */
	let level: number = LOADING;
	/** The amount of experience the user has in this level. */
	let currentXp: number = LOADING;
	/** The amount of experience required to level up. NOT relative to `currentXp`. */
	let nextXp: number = LOADING;

	let motivationalQuote = ' ';
	let percent: number = 0;
	let progressBarAnimationSpeed: number = 2000;
	let confetti = false;

	// Only set these things if not in prerender mode.
	if (browser) {
		motivationalQuote = quotes[Math.floor(Math.random() * quotes.length)];
		levelingInfo.subscribe((levelInfo) => {
			if (animate) return;

			level = levelInfo.level;
			currentXp = levelInfo.xp;
			nextXp = levelInfo.xpForNextLevel;
			percent = Math.min(currentXp / nextXp, 1);
		});
	}

	if (animate) {
		const from = getLevel(animate.fromXp);
		const to = getLevel(animate.toXp);

		level = from.level;
		currentXp = from.xp;
		nextXp = from.xpForNextLevel;
		percent = Math.min(currentXp / nextXp, 1);

		// After 1 second animate the level up.
		setTimeout(() => {
			animateXp(to);
		}, 1000);
	}

	async function animateXp(to: LevelingInfo) {
		const levelsUp = to.level > level;

		// Animate to the minimum of either the next XP or next level.
		const toXp = levelsUp ? nextXp : to.xp;

		// The progress bar is already animated using CSS
		//		`transition-[width]`   and   `transition-duration: {progressBarAnimationSpeed}ms`
		// This will provide a smoother animation than setting the width each frame.
		// Because it animates at `progressBarAnimationSpeed`ms, we use this
		// for the duration of the JS animations below.
		percent = Math.min(toXp / nextXp, 1);

		await EASING.animate(currentXp, toXp, progressBarAnimationSpeed, EASING.sineOut, (xp) => {
			currentXp = Math.floor(xp);
		});

		if (levelsUp) {
			// Wait a couple seconds for it to register to the user.
			await new Promise((resolve) => setTimeout(resolve, 1000));

			// Play confetti
			confetti = true;

			// Wait a second for the confetti to register to the user.
			await new Promise((resolve) => setTimeout(resolve, 1000));

			// Reset the level.
			progressBarAnimationSpeed = 500;
			// If levelling up more than 1 level at once this implementation
			// isnt correct but right now i cant be assed to fix it lmao.
			level = to.level;
			currentXp = 0;
			nextXp = to.xpForNextLevel;
			percent = 0;

			// Wait for that to finish + a little extra so if they're levelling
			// up more its not instant.
			await new Promise((resolve) => setTimeout(resolve, progressBarAnimationSpeed + 250));
			progressBarAnimationSpeed = 2000;

			// Call this function again.
			animateXp(to);
		}
	}
</script>

<div
	class="w-full h-[120px] flex px-8 items-center gap-10 bg-gradient-to-br from-[#f7f484] to-[#efd867] rounded-[40px]"
>
	<!-- Trophy icon -->
	<div class="flex flex-col items-center mt-1">
		<!-- the period throws off the balance so shift a little to the right -->
		<span class="text-xl text-[#54451f] -mb-5 ml-1">Lv.</span>
		<div class="relative">
			<div class="absolute inset-0 top-4 flex justify-center">
				<span class="text-5xl font-bold text-[#54451f]">{level === LOADING ? '...' : level}</span>
			</div>
			<TrophyIcon class="w-28 h-28 fill-[#fffdf0]" />
		</div>
	</div>

	<!-- Level progress -->
	<div class="flex grow flex-col gap-2">
		<div class="text-2.5xl font-bold self-end mr-4">
			<span class="text-[#54451f]">{currentXp === LOADING ? '?' : currentXp}</span>
			<span class="text-[#54451f]/80"> / {nextXp === LOADING ? '?' : nextXp}</span>
		</div>
		<div class="relative">
			<div class="h-4 w-full bg-[#fffdf0] rounded-lg" />
			<div
				class="absolute min-w-[16px] inset-y-0 bg-gradient-to-br from-[#96d0ff] to-[#74a5ed] rounded-lg transition-[width]"
				style="width: {percent * 100}%; transition-duration: {progressBarAnimationSpeed}ms"
			/>
			{#if confetti}
				<div class="absolute right-0 top-1/2">
					<Confetti cone amount={30} x={[-1, 1]} y={[0.1, 1]} />
				</div>
			{/if}
		</div>
		{#if showQuote}
			<span class="text-[#54451f]/60 text-xl italic ml-4 leading-4">{motivationalQuote}</span>
		{/if}
	</div>
</div>
