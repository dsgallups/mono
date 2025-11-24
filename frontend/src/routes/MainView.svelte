<script lang="ts">
	import { page } from '$app/state';
	import { SvelteURL } from 'svelte/reactivity';
	import MainSearch from './MainSearch.svelte';
	import type { FileSimilarity, IndexResponse } from '$lib/types';
	import { onMount } from 'svelte';

	interface Props {
		fetchIndex: () => Promise<IndexResponse[]>;
	}

	let { fetchIndex }: Props = $props();

	let searchVal = $state('');
	//let initialSearch = $state(true);
	//

	let indexResponse: IndexResponse | null = $state(null);

	let indexView = $derived.by(() => {
		if (indexResponse === null) {
			return null;
		}

		let percent = (indexResponse.progress * 100).toFixed(2);
		return {
			path: indexResponse.path,
			queue: indexResponse.queue,
			percent
		};
	});
	async function refetchIndex() {
		try {
			let result = await fetchIndex();

			if (result.length === 0) {
				return;
			}

			let last = result[result.length - 1]!;
			if (last.progress === 1) {
				clearInterval(resultInterval);
				return;
			}
			indexResponse = last;
		} catch {
			clearInterval(resultInterval);
		}
	}

	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	let resultInterval: number | undefined = $state();

	let apiUrl = new SvelteURL('/api/files', page.url);
	let fileResponse: FileSimilarity[] = $state([]);

	onMount(async () => {
		queryFiles();
		// Ideally you would spawn a websocket here.
		await refetchIndex();
		resultInterval = setInterval(refetchIndex, 1000) as unknown as number;
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
		onkeyup={() => {
			queryFiles();
		}}
	/>
	{#if indexView}
		<div class="my-3 flex max-w-full justify-between gap-2 border border-blue-400 p-3">
			<p class="wrap-anywhere">Scanning {indexView.queue}</p>
			<p>{indexView.percent}%</p>
		</div>
	{/if}
	<div>
		{#each fileResponse as file (file.id)}
			<div class="flex shrink border border-stone-400">
				<p>{file.title}</p>
			</div>
		{/each}
	</div>
</div>
