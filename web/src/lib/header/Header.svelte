<script lang="ts">
	import HeaderNavButton from './HeaderNavButton.svelte';
	import TrophyIcon from '$lib/assets/icons/rounded/trophy.svg?component';
	import SearchIcon from '$lib/assets/icons/rounded/search.svg?component';
</script>

<header class="h-[72px] bg-[#f7f7f7] w-full px-6 flex items-center">
	<!-- The navigation buttons -->
	<div class="px-8 flex space-x-10">
		<HeaderNavButton
			class="hover:text-blue-500 hover:after:bg-blue-500 focus-within:after:bg-blue-500"
			href="/"
		>
			Home
		</HeaderNavButton>
		<HeaderNavButton
			class="hover:text-fuchsia-600 hover:after:bg-fuchsia-600 focus-within:after:bg-fuchsia-600"
			href="/saved"
		>
			Saved
		</HeaderNavButton>
		<HeaderNavButton
			class="hover:text-orange-500 hover:after:bg-orange-500 focus-within:after:bg-orange-500"
			href="/rated"
		>
			Rated
		</HeaderNavButton>
		<HeaderNavButton
			class="hover:text-emerald-500 hover:after:bg-emerald-500 focus-within:after:bg-emerald-500"
			href="/list"
		>
			List
		</HeaderNavButton>
		<HeaderNavButton
			class="hover:text-yellow-600 hover:after:bg-yellow-600 focus-within:after:bg-yellow-600"
			href="/feedback"
		>
			Feedback
		</HeaderNavButton>
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
		class="ml-12 w-[70px] h-[50px] relative flex justify-center rounded-tl-2xl rounded-br-2xl rounded-tr-lg rounded-bl-lg from-[#f7f484] to-[#efd867] bg-gradient-to-br select-none"
	>
		<TrophyIcon class="relative -top-[calc((54px-50px)/2)]" fill="#fffdf0" width="54" height="54" />
		<span class="text-[#54451f] text-xl font-bold absolute top-[4px]">17</span>
	</div>
</header>
