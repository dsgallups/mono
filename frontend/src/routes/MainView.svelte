<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { SvelteURL } from 'svelte/reactivity';
	import MainSearch from './MainSearch.svelte';
	import type { FileSimilarity } from '$lib/types';
	import { onMount } from 'svelte';

	let searchVal = $state('');
	//let initialSearch = $state(true);

	function onSubmitSearch() {
		//initialSearch = false;

		const url = new URL(page.url);
		url.searchParams.set('q', searchVal);
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		goto(url, { replaceState: true, noScroll: true });
	}

	let apiUrl = new SvelteURL('/api/files', page.url);
	let fileResponse: FileSimilarity[] = $state([]);

	onMount(async () => {
		queryFiles();
	});

	async function queryFiles() {
		apiUrl.searchParams.set('q', searchVal);
		const result = await fetch(apiUrl);
		const files: FileSimilarity[] = await result.json();
		fileResponse = files;
	}
</script>

<div class="wrap flex flex-col p-5">
	<MainSearch
		bind:value={searchVal}
		onsubmit={onSubmitSearch}
		onkeyup={() => {
			queryFiles();
		}}
	/>
	<div>
		{#each fileResponse as file (file.id)}
			<div class="flex shrink border border-stone-400">
				<p>{file.title}</p>
			</div>
		{/each}
	</div>
</div>
