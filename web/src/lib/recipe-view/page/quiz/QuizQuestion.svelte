<script lang="ts">
	import _ from 'lodash';
	import Formattable from '$src/lib/utils/Formattable.svelte';
	import type { Question } from '$src/types';
	import QuizAnswer from './QuizAnswer.svelte';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let question: Question;
	$: unshuffledAnswers = [...question.correctAnswers, ...question.wrongAnswers];
	// Give each a randomly assigned ID so that the answer components dont
	// stay across questions.
	$: shuffledAnswers = _.shuffle(unshuffledAnswers).map((answer) => {
		return { answer, id: Math.random() };
	});

	let showAnswers = false;
	function clicked() {
		showAnswers = true;
	}

	// Reset showAnswers when the question changes
	$: {
		question;
		showAnswers = false;
	}
</script>

<div class="my-4 w-full">
	<div>
		{#if question.image}
			<img
				src={question.image}
				class="rounded-3xl object-cover aspect-[3/2] w-1/3 h-fit float-right ml-4 mb-4"
			/>
		{/if}
		<h3 class="font-bold text-3xl justify-self-center text-center mb-2">{question.question}</h3>
		{#if question.description}
			<p class="text-xl"><Formattable content={question.description} /></p>
		{/if}
	</div>

	<div
		class="mt-8 grid grid-cols-2 auto-rows-auto gap-[10px_20px] items-center w-full {showAnswers
			? 'show-answers'
			: ''}"
	>
		{#each shuffledAnswers as answer (answer.id)}
			<QuizAnswer
				content={answer.answer}
				correct={question.correctAnswers.includes(answer.answer)}
				{showAnswers}
				on:correct={() => dispatch('click', true)}
				on:wrong={() => dispatch('click', false)}
				on:click={clicked}
			/>
		{/each}
	</div>
</div>
