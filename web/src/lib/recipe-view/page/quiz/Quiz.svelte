<script lang="ts">
	import _ from 'lodash';
	import QuizIcon from '$icons/quiz.svg?component';
	import Dialog from '$lib/utils/Dialog.svelte';
	import QuizQuestion from './QuizQuestion.svelte';
	import ArrowForwardIcon from '$icons/arrow-forward.svg?component';

	import type { Recipe } from '$types/Recipe';
	import { fade } from 'svelte/transition';

	export let recipe: Recipe;
	export let open: boolean;

	let nextQuestionButton = false;

	let correctCount = 0;
	let totalReward = 0;

	let qIndex = 0;
	let questions = _.shuffle(recipe.quiz.questions);
	let question = questions[qIndex];

	function answered(e: CustomEvent<boolean>) {
		const correct = e.detail;
		correctCount += correct ? 1 : 0;
		totalReward += correct ? question.reward : 0;

		// Wait 0.5 second before showing the next question button.
		setTimeout(() => {
			nextQuestionButton = true;
		}, 500);
	}

	function nextQuestion() {
		nextQuestionButton = false;
		if (qIndex + 1 >= questions.length) {
			console.log('quiz done');
			open = false;
		} else {
			qIndex++;
			question = questions[qIndex];
		}
	}
</script>

<Dialog {open} titleGradient={['#f7f484', '#efd867']} on:close>
	<QuizIcon slot="icon" class="w-full h-full fill-black/80" />
	<span slot="title" class="text-black/90">Quiz</span>
	<div slot="content" class="w-full relative">
		{#if nextQuestionButton}
			<!--
				Very messy here.
				Essentially, try place the button with its center on the edge of the
				popup, while making sure misclicks on the right side will not close
				the quiz, and any clicks on the content (the left) or button
				will skip to the next question.
			-->
			<div class="absolute inset-y-0 right-0 translate-x-full pr-20 flex flex-col justify-center">
				<!-- Because of the padding on the wrapper to make sure the
					player doesnt misclick, clicking the below also breaks.
					Thus, we place the z-ordering here. -->
				<button
					in:fade={{ duration: 100 }}
					class="absolute z-20 rounded-full -translate-x-1/2 aspect-square bg-[#f7f484] hover:bg-[#efd867] transition-colors shadow-lg p-3"
					on:click={nextQuestion}
				>
					<ArrowForwardIcon class="fill-black/80 w-10 h-10" />
				</button>
			</div>
			<!-- Higher Z than content but lower Z than button so its still clickable. -->
			<div class="absolute z-10 inset-0" on:click={nextQuestion} />
		{/if}
		<div class="px-12">
			<QuizQuestion {question} on:click={answered} />
		</div>
	</div>
</Dialog>
