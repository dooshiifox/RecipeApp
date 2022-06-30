<script lang="ts">
	import { page } from '$app/stores';
	import TrophyIcon from '$lib/assets/icons/rounded/trophy.svg?component';
	import SearchIcon from '$lib/assets/icons/rounded/search.svg?component';
</script>

<header class="h-[72px] bg-[#f7f7f7] w-full px-6 flex items-center">
	<!-- The navigation buttons -->
	<div class="px-8 flex text-black/60 text-xl space-x-10">
		<p>Home</p>
		<p>Saved</p>
		<p>Rated</p>
		<p>List</p>
		<p>Feedback</p>
	</div>
	<!-- Grows the distance between the navbuttons and searchbar
			only if max width of searchbar is reached. -->
	<span class="mx-auto" />
	<!-- The search bar -->
	<div class="flex grow max-w-[500px]">
		<div
			class="rounded-xl h-10 w-full py-1 px-2 flex justify-center items-center bg-white border-black/5 border"
		>
			<SearchIcon class="ml-1 w-6 h-6 text-black/50" />
			<input
				type="text"
				class="ml-3 w-full h-full text-xl outline-none border-b border-b-transparent focus:border-b-black/50 transition-[border-bottom-color] duration-100"
				placeholder="Search for Recipes"
			/>
		</div>
	</div>
	<!-- The trophy with the yellow background. -->
	<!-- Gross use of absolute values here but it makes it look better. -->
	<div
		class="ml-12 w-[70px] h-[50px] relative flex justify-center rounded-tl-2xl rounded-br-2xl rounded-tr-lg rounded-bl-lg from-[#f7f484] to-[#efd867] bg-gradient-to-br"
	>
		<TrophyIcon class="relative -top-[calc((54px-50px)/2)]" fill="#fffdf0" width="54" height="54" />
		<span class="text-[#54451f] text-xl font-bold absolute top-[4px]">17</span>
	</div>
</header>
