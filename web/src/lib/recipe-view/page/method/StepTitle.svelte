<script lang="ts">
	export let index: number;
	export let title: string;
</script>

<div class="flex items-center gap-8 pl-10 pr-4 pb-2">
	<span class="text-[42px] font-bold italic text-black/70">{index}.</span>
	<span class="text-3xl font-bold text-black/80">{title}</span>
</div>
