<script lang="ts">
	import Formattable from '$src/lib/Formattable.svelte';
	import type { Info } from '$types/Recipe';
	import InfoIcon from '$icons/menu-book.svg?component';

	export let info: Info;
</script>

<div class="bg-[#7fdd9a] rounded-3xl px-6 py-3 pb-4">
	<div class="flex items-center gap-4">
		<InfoIcon class="w-10 h-10 fill-black/70" />
		<span class="text-2.5xl font-bold text-black/70">{info.title}</span>
	</div>
	<div class="mt-3 px-8 italic text-black/80 text-xl">
		<Formattable content={info.content} />
	</div>
</div>
