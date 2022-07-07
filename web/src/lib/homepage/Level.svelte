<script lang="ts">
	import { browser } from '$app/env';
	import TrophyIcon from '$lib/assets/icons/rounded/trophy.svg?component';
	import getLevelingInfo from '../../store/level';

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

	let level: number | string = '...';
	let currentXp: number | string = '?';
	let nextXp: number | string = '?';
	let motivationalQuote = ' ';

	$: percent =
		typeof currentXp === 'number' && typeof nextXp === 'number'
			? Math.min(currentXp / nextXp, 1)
			: 0;

	// Only set these things if not in prerender mode.
	if (browser) {
		let levelInfo = getLevelingInfo();

		level = levelInfo.level;
		currentXp = levelInfo.xp;
		nextXp = levelInfo.xpForNextLevel;
		motivationalQuote = quotes[Math.floor(Math.random() * quotes.length)];
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
				<span class="text-5xl font-bold text-[#54451f]">{level}</span>
			</div>
			<TrophyIcon class="w-28 h-28 fill-[#fffdf0]" />
		</div>
	</div>

	<!-- Level progress -->
	<div class="flex grow flex-col gap-2">
		<div class="text-2.5xl font-bold self-end mr-4">
			<span class="text-[#54451f]">{currentXp}</span>
			<span class="text-[#54451f]/80"> / {nextXp}</span>
		</div>
		<div class="relative">
			<div class="h-4 w-full bg-[#fffdf0] rounded-lg" />
			<div
				class="absolute min-w-[16px] inset-y-0 bg-gradient-to-br from-[#96d0ff] to-[#74a5ed] rounded-lg transition-[width] duration-2000"
				style="width: {percent * 100}%;"
			/>
		</div>
		<span class="text-[#54451f]/60 text-xl italic ml-4 leading-4">{motivationalQuote}</span>
	</div>
</div>
