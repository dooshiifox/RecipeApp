<script lang="ts">
	import Star from '$lib/assets/icons/rounded/star.svg?component';
	import HalfStar from '$lib/assets/icons/rounded/star-half.svg?component';
	import EmptyStar from '$lib/assets/icons/rounded/star-border.svg?component';

	export let stars: number;
	export let starClass: string;
	let className: string;
	export { className as class };
</script>

<div class="flex {className}">
	{#each Array(5) as _, i}
		<span>
			{#if stars === i + 0.5}
				<HalfStar class={starClass} />
			{:else if stars >= i + 1}
				<Star class={starClass} />
			{:else}
				<EmptyStar class={starClass} />
			{/if}
		</span>
	{/each}
</div>
