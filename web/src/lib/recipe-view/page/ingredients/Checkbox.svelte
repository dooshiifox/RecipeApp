<script lang="ts">
	import TickedCheckboxIcon from '$icons/checkbox-ticked.svg?component';
	import EmptyCheckboxIcon from '$icons/checkbox-empty.svg?component';

	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	let className: string = '';
	export { className as class };

	export let ticked: boolean = false;

	function toggleTicked(): void {
		ticked = !ticked;
		dispatch('toggle', {
			ticked
		});
	}
</script>

<div on:click|preventDefault|stopPropagation={toggleTicked}>
	{#if ticked}
		<TickedCheckboxIcon class="active {className}" />
	{:else}
		<EmptyCheckboxIcon class="inactive {className}" />
	{/if}
</div>
