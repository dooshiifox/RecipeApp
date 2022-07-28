<script lang="ts">
	import QuizIcon from '$icons/quiz.svg?component';
</script>

<div class="relative py-6">
	<div class="absolute inset-0 -z-10 skew-y-2 bg-[#faf8c4]" />
	<div class="mb-4 text-black/80 w-96 text-xl mx-auto text-center">
		Play the quiz to show your knowledge and increase your level!
	</div>
	<button
		on:click
		class="mx-auto px-12 py-4 rounded-3xl text-3xl font-bold text-black/80 flex items-center gap-8 outline-none bg-[#efd867] hover:bg-[#f1df80] ring-[#ceb537] focus:ring-2 transition-colors"
	>
		<QuizIcon class="w-10 h-10 fill-black/70" />
		Take the Quiz
	</button>
</div>
