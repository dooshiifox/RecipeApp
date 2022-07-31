<script lang="ts">
	import _ from 'lodash';
	import QuizIcon from '$icons/quiz.svg?component';
	import Dialog from '$lib/utils/Dialog.svelte';
	import QuizQuestion from './QuizQuestion.svelte';
	import ArrowForwardIcon from '$icons/arrow-forward.svg?component';

	import type { Recipe } from '$types/Recipe';
	import { fade } from 'svelte/transition';
	import Level from '$src/lib/homepage/Level.svelte';
	import { getTotalXp, setTotalXp } from '$src/store/level';
	import type { Question } from '$src/types';

	export let recipe: Recipe;
	export let open: boolean;

	const xpStartingQuiz = getTotalXp();

	let nextQuestionButton = false;

	enum AnswerState {
		Correct,
		Incorrect,
		Unanswered,
		Current
	}

	// An array of questions in order.
	// Shuffles the given questions so theyre not in order and it feels
	// more random.
	let questions: { question: Question; state: AnswerState }[] = _.shuffle(
		recipe.quiz.questions
	).map((question) => {
		return { question, state: AnswerState.Unanswered };
	});

	let qIndex = 0;
	let question = questions[qIndex].question;
	questions[qIndex].state = AnswerState.Current;

	let totalReward = 0;
	/** The number of questions answered correctly. */
	$: correctCount = questions.filter((q) => q.state === AnswerState.Correct).length;

	// Whether to show the end page or the close button.
	let showEnd = false;
	let showClose = false;

	function answered(e: CustomEvent<boolean>) {
		const correct = e.detail;
		if (correct) {
			totalReward += question.reward;
			setTotalXp(getTotalXp() + question.reward);
		}

		// Update state of the question to reflect the answer.
		questions[qIndex].state = correct ? AnswerState.Correct : AnswerState.Incorrect;

		// Wait 0.5 second before showing the next question button.
		setTimeout(() => {
			nextQuestionButton = true;
		}, 500);
	}

	function nextQuestion() {
		nextQuestionButton = false;
		if (qIndex + 1 >= questions.length) {
			onFinish();
		} else {
			qIndex++;
			question = questions[qIndex].question;
			questions[qIndex].state = AnswerState.Current;
		}
	}

	function keyPress(e: KeyboardEvent) {
		if (!nextQuestionButton) return;
		if (e.key === 'Enter') {
			e.stopPropagation();
			e.preventDefault();
			nextQuestion();
		}
	}

	function onFinish() {
		showEnd = true;

		// Check if the user got all of them correct.
		if (correctCount === questions.length) {
			totalReward += recipe.quiz.allCorrectReward;
			setTotalXp(getTotalXp() + recipe.quiz.allCorrectReward);
		}

		// Wait 0.5 seconds before showing the close button
		setTimeout(() => {
			showClose = true;
		}, 500);
	}
</script>

<Dialog {open} titleGradient={['#f7f484', '#efd867']} on:close>
	<QuizIcon slot="icon" class="w-full h-full fill-black/80" />
	<span slot="title" class="text-black/90">Quiz</span>
	<div slot="content" class="w-full relative">
		<!-- Question counter. -->
		<div class="absolute -top-8 right-0 flex gap-2 p-4">
			{#each questions as q}
				<div
					class="w-4 h-4 rounded"
					class:bg-lime-300={q.state === AnswerState.Correct}
					class:bg-red-300={q.state === AnswerState.Incorrect}
					class:bg-blue-300={q.state === AnswerState.Current}
					class:bg-gray-400={q.state === AnswerState.Unanswered}
				/>
			{/each}
		</div>

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
		<div class="px-12" on:keypress={keyPress}>
			{#if showEnd}
				<div class="text-center mt-8">
					<h4 class="text-black/90 text-4xl font-bold">Quiz Complete!</h4>
					<div class="text-black/70 text-2xl my-6">
						<p>You got</p>
						<p class="mt-2">
							<span class="text-[#efd867] text-8xl font-bold">{correctCount}&nbsp;</span>
							<span class="text-[#efd867] text-7xl font-bold">/ {questions.length}</span>
						</p>
					</div>
					<Level
						showQuote={false}
						animate={{ fromXp: xpStartingQuiz, toXp: xpStartingQuiz + totalReward }}
					/>
				</div>
			{:else}
				<QuizQuestion {question} on:click={answered} />
			{/if}
		</div>
	</div>
	<div slot="close">
		{#if showEnd}
			<button
				class="rounded-full w-[480px] bg-[#f7f484] hover:bg-[#efd867] shadow-lg p-3 transition-all"
				class:opacity-0={!showClose}
				on:click={() => (open = false)}
			>
				<span class="text-3xl font-bold text-black/90">Close</span>
			</button>
		{/if}
	</div>
</Dialog>
