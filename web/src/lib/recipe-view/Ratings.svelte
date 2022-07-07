<script lang="ts">
	import Star from '$icons/star.svg?component';
	import HalfStar from '$icons/star-half.svg?component';
	import EmptyStar from '$icons/star-border.svg?component';

	/** The number of stars to show.
	 * If undefined, does not show component but consumes width.
	 */
	export let stars: number | undefined;
	export let starClass: string = '';
	let className: string = '';
	export { className as class };
</script>

<div class="flex {className}">
	{#each Array(5) as _, i}
		<span>
			{#if stars === undefined}
				<EmptyStar class="{starClass} invisible" />
			{:else if stars === i + 0.5}
				<HalfStar class={starClass} />
			{:else if stars >= i + 1}
				<Star class={starClass} />
			{:else}
				<EmptyStar class={starClass} />
			{/if}
		</span>
	{/each}
</div>
