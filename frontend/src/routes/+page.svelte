<script lang="ts">
	import { page } from '$app/state';
	import DirSearch from './DirSearch.svelte';
	import type { IndexResponse } from '$lib/types';
	import MainView from './MainView.svelte';
	import { onMount } from 'svelte';

	let loading = $state(true);
	let indexResponse: IndexResponse[] = $state([]);
	onMount(async () => {
		indexResponse = await fetchIndex();
		loading = false;
	});

	let refetchIndex: number | undefined = $state();

	async function fetchIndex() {
		const url = new URL('/api/index_tasks', page.url);
		const result = await fetch(url);
		const response: IndexResponse[] = await result.json();
		return response;
	}

	async function onSubmitDirSearch(value: string) {
		console.log('submitting value ', value);
		const url = new URL('/api/index_tasks', page.url);

		const response = await fetch(url, {
			method: `POST`,
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				path: value
			})
		});
		console.log('HERE!');
		if (response.ok) {
			console.log('refetching index');
			refetchIndex = setInterval(async () => {
				let response = await fetchIndex();
				if (response.length !== 0) {
					indexResponse = response;
					clearInterval(refetchIndex);
				}
			}, 500) as unknown as number;
		}
	}
</script>

<div class="flex">
	{#if loading}
		<p>Loading State</p>
	{:else if indexResponse.length === 0}
		<div class="flex justify-center p-5">
			<div class="flex flex-col gap-2">
				<p class="text-lg">Let's get to indexing!</p>
				<DirSearch onsubmit={onSubmitDirSearch} />
			</div>
		</div>
	{:else}
		<MainView {fetchIndex} {onSubmitDirSearch} />
	{/if}
</div>
