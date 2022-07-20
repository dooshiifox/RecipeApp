<script lang="ts">
	import Formattable from '$src/lib/Formattable.svelte';
	import type { Warning } from '$types/Recipe';
	import WarningIcon from '$icons/warning.svg?component';

	export let warning: Warning;
</script>

<div class="bg-[#efbd54] rounded-3xl px-6 py-3 pb-4">
	<div class="flex items-center gap-4">
		<WarningIcon class="w-10 h-10 fill-black/70" />
		<span class="text-2.5xl font-bold text-black/70">{warning.title}</span>
	</div>
	<div class="mt-3 px-8 italic text-black/80 text-xl">
		<Formattable content={warning.content} />
	</div>
</div>
