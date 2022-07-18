<script lang="ts">
	import BookmarkIcon from '$icons/bookmark.svg?component';
	import EmptyBookmarkIcon from '$icons/bookmark-border.svg?component';
	import { browser } from '$app/env';
	import type { BasicRecipe } from '$src/types/BasicRecipe';

	export let recipe: BasicRecipe;
	/** The classes on the invisible clickable wrapper. */
	export let invisWrapperClass: string;
	/** The classes on the visible wrapper. */
	export let wrapperClass: string;
	/** The classes on the icons themselves. */
	let className: string;
	export { className as class };

	function bookmark() {
		recipe.bookmarked = !recipe.bookmarked;
	}
</script>

<div
	class="{invisWrapperClass} grid place-items-center bookmark-wrapper"
	on:click|stopPropagation|preventDefault={bookmark}
>
	<div class="{wrapperClass} bookmark-wrapper-hover:bg-white/30 transition-all">
		{#if browser && recipe.bookmarked}
			<BookmarkIcon class={className} />
		{:else}
			<EmptyBookmarkIcon class={className} />
		{/if}
	</div>
</div>

<style>
	.bookmark-wrapper:hover .bookmark-wrapper-hover\:bg-white\/30 {
		background-color: rgb(255 255 255 / 0.3);
	}
</style>
