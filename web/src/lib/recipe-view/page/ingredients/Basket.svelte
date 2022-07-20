<script lang="ts">
	import FullBasketIcon from '$icons/basket-full.svg?component';
	import EmptyBasketIcon from '$icons/basket-empty.svg?component';

	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	let className: string = '';
	export { className as class };

	export let basketed: boolean = false;

	function toggleBasketed(): void {
		basketed = !basketed;
		dispatch('toggle', {
			basketed
		});
	}
</script>

<div on:click|preventDefault|stopPropagation={toggleBasketed}>
	{#if basketed}
		<FullBasketIcon class="active {className}" />
	{:else}
		<EmptyBasketIcon class="inactive {className}" />
	{/if}
</div>
