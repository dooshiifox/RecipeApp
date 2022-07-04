<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let selected: boolean = false;

	function toggleSelection(): void {
		selected = !selected;
		dispatch('toggleSelection', {
			selected
		});
	}
</script>

<button
	class="px-4 h-8 rounded-2xl flex justify-center items-center group {selected
		? 'bg-white/80'
		: 'bg-white/20'}"
	on:click={() => toggleSelection()}
>
	<span class="{selected ? 'text-black/70' : 'text-white/70'} text-xl group-focus:border-b-black">
		<slot />
	</span>
</button>
