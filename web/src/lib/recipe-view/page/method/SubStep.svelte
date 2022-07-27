<script lang="ts">
	import Formattable from '$src/lib/utils/Formattable.svelte';
	import type { Step, SubStep } from '$types/index';
	import Warning from './Warning.svelte';
	import Info from './Info.svelte';

	export let step: Step;
	export let index: number;

	// `undefined` when loading in? Not sure why but i think its a server thing.
	$: substep = step.substeps[index] as SubStep | undefined;

	$: image = substep?.image;

	// Flip the side the image is on each time an image is present
	// in the substeps.
	// Initially start on the right side.
	$: imageOnLeft = (() => {
		let isOnLeft = false;
		if (image) {
			for (let i = 0; i < index; i++) {
				if (step.substeps[i].image !== null) {
					isOnLeft = !isOnLeft;
				}
			}
		}
		return isOnLeft;
	})();

	$: firstSubstep = index === 0;
	$: finalSubstep = index === step.substeps.length - 1;
	$: onlySubstep = firstSubstep && finalSubstep;

	$: roundedImage = (() => {
		if (onlySubstep) return 'rounded-3xl';
		if (firstSubstep || finalSubstep) {
			const tlbr = 'rounded-tl-3xl rounded-br-3xl';
			const trbl = 'rounded-tr-3xl rounded-bl-3xl';
			if ((firstSubstep && imageOnLeft) || (finalSubstep && !imageOnLeft)) {
				return tlbr;
			} else {
				return trbl;
			}
		}
		if (imageOnLeft) return 'rounded-r-3xl';
		return 'rounded-l-3xl';
	})();

	$: imgMarginClasses = (() => {
		if (onlySubstep) return '';
		if (firstSubstep) return 'mb-2';
		if (finalSubstep) return 'mt-2';
		return 'my-2';
	})();
	$: textMarginClasses = (() => {
		if (!image) {
			let mt = 'mt-1';
			let mb = 'mb-1';
			// If the previous substep has an image,
			//  then the text should be pushed further down.
			// Don't do it if its substep #1, because that's where the title
			//  goes.
			if (!firstSubstep && step.substeps[index - 1].image !== null) {
				mt = 'mt-4';
			}
			// If the next substep has an image, or its the last substep,
			//  then the text should be pushed further up.
			if (finalSubstep || step.substeps[index + 1].image !== null) {
				mb = 'mb-4';
			}
			return `${mt} ${mb}`;
		} else {
			if (firstSubstep) return 'mb-4';
			if (finalSubstep) return 'mt-4';
			return 'my-4';
		}
	})();
</script>

<div class="flex flex-col w-full">
	<div class="flex {imageOnLeft ? 'flex-row' : 'flex-row-reverse justify-end'} items-center">
		{#if image}
			<img
				src={image}
				class="w-[300px] aspect-[3/2] {roundedImage} {imgMarginClasses} bg-white/10 {onlySubstep
					? ''
					: firstSubstep
					? 'self-start'
					: finalSubstep
					? 'self-end'
					: ''}"
				alt="Image of substep {index}"
			/>
		{/if}
		<div
			class="grow text-2xl text-black/80 mx-8 {textMarginClasses} {firstSubstep && image
				? 'self-start'
				: ''}"
		>
			<slot name="title" />
			<Formattable content={substep?.content ?? ''} />
		</div>
	</div>
	<div
		class="flex flex-col gap-4 w-2/3 relative {!image
			? index % 2
				? '-left-40'
				: '-right-40'
			: imageOnLeft
			? '-right-40'
			: '-left-40'}"
	>
		{#each substep?.warnings ?? [] as warning}
			<Warning {warning} />
		{/each}
	</div>
	<div
		class="flex flex-col gap-4 w-2/3 relative {!image
			? index % 2
				? '-left-40'
				: '-right-40'
			: imageOnLeft
			? '-right-40'
			: '-left-40'}"
	>
		{#each substep?.infos ?? [] as info}
			<Info {info} />
		{/each}
	</div>
</div>
