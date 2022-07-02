<script lang="ts">
	import SearchIcon from '$lib/assets/icons/rounded/search.svg?component';
	import FilterIcon from '$lib/assets/icons/rounded/filter.svg?component';
	import ListIcon from '$lib/assets/icons/rounded/list.svg?component';
</script>

<div class="relative w-full pt-8 pb-10 flex flex-col items-center justify-center gap-6">
	<!-- titled background -->
	<div class="absolute inset-0 -z-10 -skew-y-2 bg-[#ebebed]" />
	<div class="flex flex-col items-center justify-center">
		<h5 class="text-2xl font-bold text-black/90">Don't see anything you like?</h5>
		<p class="text-lg text-black/60 text-center leading-5">
			Try
			<span class="font-bold text-[#ad7bd8] fill-[#ad7bd8]">
				<SearchIcon class="inline w-[20px] h-[20px] ml-0.5" />searching
			</span>
			for recipes you like a lot, or
			<br />
			<span class="font-bold text-[#688edd] fill-[#688edd] text-center">
				<FilterIcon class="inline w-[20px] h-[20px] mr-1 -mt-1" />filter
			</span>
			out things you don't like.
		</p>
	</div>
	<div class="flex flex-col items-center justify-center">
		<h5 class="text-2xl font-bold text-black/90">Enjoy cooking your own ideas?</h5>
		<p class="text-lg text-black/60 text-center leading-5">
			Why not make your own recipe and
			<br />
			send it to us using the <span class="text-[#3cb946] font-bold">green button</span> below!
		</p>
	</div>
	<a
		href="/suggest-recipe"
		class="px-6 py-3 bg-gradient-to-br from-[#64e16c] to-[#77e8ad] rounded-2xl flex items-center justify-center gap-4"
	>
		<ListIcon class="w-8 h-8 fill-[#233e2a]" />
		<span class="text-[#233e2a] text-2xl font-bold">Suggest a Recipe</span>
	</a>
</div>
