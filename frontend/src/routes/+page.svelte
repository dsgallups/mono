<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { SvelteURL } from 'svelte/reactivity';
	import DirSearch from './DirSearch.svelte';
	import MainSearch from './MainSearch.svelte';
	import type { FileResponse } from '$lib/types';

	let initialSearch = $state(true);

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

	let searchVal = $state('');

	let dirSearchFocused = $state(false);

	function onSubmitSearch() {
		initialSearch = false;

		const url = new URL(page.url);
		url.searchParams.set('q', searchVal);
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		goto(url, { replaceState: true, noScroll: true });
	}

	async function onSubmitDirSearch(value: string) {
		initialSearch = false;
		console.log('submitting value ', value);
		const url = new URL('/api/index_tasks', page.url);

		await fetch(url, {
			method: `POST`,
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				path: value
			})
		});
	}
</script>

<div class="flex">
	{#if initialSearch}
		<div class="flex justify-center p-5">
			<div class="flex flex-col gap-2">
				<DirSearch
					onfocus={() => {
						console.log('focusin');
						dirSearchFocused = true;
					}}
					onblur={() => {
						console.log('blurin');
						dirSearchFocused = false;
					}}
					onsubmit={onSubmitDirSearch}
				/>
				{#if !dirSearchFocused}
					<div>
						<MainSearch bind:value={searchVal} onsubmit={onSubmitSearch} />
					</div>
				{/if}
			</div>
		</div>
	{:else}
		<div class="flex p-5">
			<MainSearch bind:value={searchVal} onsubmit={onSubmitSearch} />
		</div>
		{#await data then data}
			<div>{data.files.length}</div>
		{/await}
	{/if}
</div>
