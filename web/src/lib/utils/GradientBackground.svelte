<script lang="ts">
	/* Either `gradient` or `starting` and `ending` must be defined. */
	import type { Gradient, Color } from '$types/gradient';

	export let gradient: Gradient | undefined = undefined;
	export let starting: Color | undefined = undefined;
	export let ending: Color | undefined = undefined;

	let className: string = '';
	export { className as class };

	if (!gradient && !(starting && ending)) {
		throw new Error('Either `gradient` or `starting` and `ending` must be defined.');
	}

	$: a = starting ?? gradient![0];
	$: b = ending ?? gradient![1];
</script>

<div
	class="{className} bg-gradient-to-br from-[var(--gradient-1)] to-[var(--gradient-2)]"
	style="--gradient-1:{a}; --gradient-2:{b}"
>
	<slot />
</div>
