<script lang="ts">
	import { page } from '$app/state';
	import type { FileSimilarity } from '$lib/types';
	import { onMount } from 'svelte';

	let file: FileSimilarity | undefined = $state();

	onMount(async () => {
		const chunkParam = page.url.searchParams.get('chunk');
		const chunkId = chunkParam ? parseInt(chunkParam) : null;

		const url = new URL(`/api/file/${page.params.id}`, page.url);
		if (chunkId !== null) {
			url.searchParams.set('chunk', `${chunkId}`);
		}

		const response = await fetch(url);
		const data: FileSimilarity = await response.json();
		file = data;
	});
</script>
