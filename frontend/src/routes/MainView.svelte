<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { SvelteURL } from 'svelte/reactivity';
	import MainSearch from './MainSearch.svelte';
	import type { FileResponse } from '$lib/types';

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
	let data = $derived.by(async () => {
		let qParams = apiUrl.searchParams.get('q');
		if (!qParams) {
			return {
				files: []
			};
		}
		const result = await fetch(apiUrl);
		const files: FileResponse[] = await result.json();
		return {
			files
		};
	});
</script>

<div class="flex p-5">
	<MainSearch bind:value={searchVal} onsubmit={onSubmitSearch} />
</div>
