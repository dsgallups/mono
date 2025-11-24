<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="flex h-dvh w-dvw flex-1 flex-col items-center">
	<h1 class="pt-4 text-5xl">Mono</h1>
	<div class="flex flex-1">
		{@render children()}
	</div>
</div>
