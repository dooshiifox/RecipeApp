<script lang="ts">
	import GradientBackground from '$lib/utils/GradientBackground.svelte';
	import type { Gradient } from '$types/gradient';
	import type { Step } from '$types/index';
	import StepTitle from './StepTitle.svelte';
	import SubStep from './SubStep.svelte';

	export let step: Step;
	export let index: number;
	export let gradient: Gradient;
</script>

<div class="relative rounded-3xl">
	<GradientBackground
		{gradient}
		class="absolute inset-0 saturate-[0.3] brightness-[1.15] rounded-3xl -z-10"
	/>
	<div class="flex flex-col">
		{#each Array(step.substeps.length) as _, i}
			{#if i === 0}
				<SubStep {step} index={i}>
					<div slot="title" class="mt-6 mb-4">
						<StepTitle {index} title={step.title} />
					</div>
				</SubStep>
			{:else}
				<SubStep {step} index={i} />
			{/if}
		{/each}
	</div>
</div>
