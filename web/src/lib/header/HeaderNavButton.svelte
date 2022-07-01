<script lang="ts">
	import { page } from '$app/stores';

	// Workaround for not being able to do `export let class: string`
	// https://navillus.dev/blog/svelte-class-props/
	let className: string;
	export { className as class };

	export let href: string;
	export let pathFn: (path: string) => boolean = (path) => path === href;
</script>

<p
	class="{className} hover:after:h-0.5 focus-within:after:h-0.5 {pathFn($page.url.pathname)
		? 'text-black after:bg-black/50 after:h-0.5'
		: 'text-black/60 after:h-0 after:bg-transparent'} text-xl relative after:absolute after:bottom-0 after:left-0 after:right-0 transition-colors duration-200 after:transition-all after:duration-200"
>
	<a {href} class="outline-none "><slot /></a>
</p>
