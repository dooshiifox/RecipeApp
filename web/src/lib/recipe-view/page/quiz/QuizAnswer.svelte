<script lang="ts">
	import Formattable from '$src/lib/utils/Formattable.svelte';
	import type { Formattable as FormattableType } from '$src/types';
	import { createEventDispatcher } from 'svelte';
	import { Confetti } from 'svelte-confetti';

	const dispatch = createEventDispatcher();

	export let content: FormattableType;
	export let correct: boolean;
	export let showAnswers: boolean;

	let confetti = false;

	let wasClicked = false;
	function clicked() {
		// Don't do anything if another answer was clicked.
		if (showAnswers) return;

		wasClicked = true;
		dispatch('click');
		if (correct) {
			confetti = true;
			dispatch('correct', { content });
		} else {
			dispatch('wrong', { content });
		}
	}

	$: classes = (() => {
		if (wasClicked) {
			if (correct) return 'bg-green-500 text-white font-bold';
			else return 'bg-red-500 text-white font-bold';
		} else if (showAnswers) {
			if (correct) return 'bg-green-200';
			else return 'bg-red-200';
		}

		return 'bg-black/10 hover:bg-black/20';
	})();
</script>

<div class="relative">
	{#if confetti}
		<div class="absolute left-1/2 top-1/2">
			<Confetti cone amount={30} x={[-1, 1]} y={[0.1, 1]} />
		</div>
	{/if}
	<button
		class="w-full rounded-3xl text-xl py-2 px-4 transition-colors {classes}"
		on:click={clicked}
	>
		<p class="break-words"><Formattable {content} /></p>
	</button>
</div>
