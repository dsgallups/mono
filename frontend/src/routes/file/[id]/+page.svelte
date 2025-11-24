<script lang="ts">
	import { page } from '$app/state';
	import type { FileDetails } from '$lib/types';
	import { onMount } from 'svelte';

	let file: FileDetails | undefined = $state();

	onMount(async () => {
		const chunkParam = page.url.searchParams.get('chunk');
		const chunkId = chunkParam ? parseInt(chunkParam) : null;

		const url = new URL(`/api/file/${page.params.id}`, page.url);
		if (chunkId !== null) {
			url.searchParams.set('chunk', `${chunkId}`);
		}

		const response = await fetch(url);
		const data: FileDetails = await response.json();
		file = data;
	});
</script>
