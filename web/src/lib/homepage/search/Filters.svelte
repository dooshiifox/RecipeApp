<script lang="ts">
	import FilterIcon from '$icons/filter.svg?component';
	import { createEventDispatcher } from 'svelte';
	import type { Filter } from './filters';
	import FilterTag from './FilterTag.svelte';

	const dispatch = createEventDispatcher();

	/** A collection of all possible filters. */
	export let filters: Filter[];

	/** All selected elements. Record of <filterList.title, selected filterList.items> */
	export let selectedFilters: Record<string, Set<string>> = {};

	/** Toggles the selection state of an item. */
	function setSelection(filter: string, item: string, setSelected: boolean): void {
		if (!selectedFilters[filter]) selectedFilters[filter] = new Set();

		if (setSelected) {
			selectedFilters[filter].add(item);
		} else {
			selectedFilters[filter].delete(item);
		}

		dispatch('change', { filter, item, setSelected });
	}

	/** Gets the selection state of a filter item. */
	function getSelection(filter: string, item: string): boolean {
		return selectedFilters[filter]?.has(item);
	}
</script>

<div class="w-[800px] bg-black/30 rounded-4xl pt-4 px-12 pb-8">
	<p class="flex items-center justify-center gap-4">
		<FilterIcon class="w-9 h-9 fill-white/90 inline" />
		<span class="text-white/80 font-bold text-4xl">Filters</span>
	</p>
	<div class="flex flex-col gap-6">
		{#each filters as filter}
			<div class="flex flex-col gap-3">
				<h4 class="text-3xl font-bold text-white/80">{filter.title}</h4>
				<div class="flex items-center flex-wrap">
					{#each filter.items as item}
						<FilterTag
							class="mr-4 mb-2"
							selected={getSelection(filter.title, item)}
							on:toggleSelection={(e) => setSelection(filter.title, item, e.detail.selected)}
						>
							{item}
						</FilterTag>
					{/each}
				</div>
			</div>
		{/each}
	</div>
</div>
