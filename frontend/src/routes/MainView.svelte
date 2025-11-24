<script lang="ts">
	import { page } from '$app/state';
	import { SvelteURL } from 'svelte/reactivity';
	import MainSearch from './MainSearch.svelte';
	import type { FileSimilarity, IndexResponse } from '$lib/types';
	import { onMount } from 'svelte';
	import FileCard from './FileCard.svelte';
	import DirSearch from './DirSearch.svelte';

	interface Props {
		fetchIndex: () => Promise<IndexResponse[]>;
		onSubmitDirSearch: (arg0: string) => void;
	}

	let { fetchIndex, onSubmitDirSearch }: Props = $props();

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
			id: indexResponse.id,
			path: indexResponse.path,
			queue: indexResponse.queue,
			percent
		};
	});

	let attempts = $state(0);

	async function refetchIndex() {
		try {
			let result = await fetchIndex();

			if (result.length === 0) {
				return;
			}

			for (const index of result) {
				if (index.progress === 1) {
					continue;
				}

				indexResponse = index;
				attempts = 0;
				return;
			}

			clearInterval(resultInterval);
		} catch {
			attempts += 1;
			if (attempts >= 5) {
				clearInterval(resultInterval);
			}
		}
	}

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

	async function cancelIndex(id: number) {
		const url = new URL(`/api/index_tasks/${id}`, page.url);
		await fetch(url, {
			method: `DELETE`
		});
	}

	let showDirSearch = $state(true);
</script>

<div class="wrap flex flex-col p-5">
	<div class="flex shrink flex-col items-start self-center">
		<div class="flex items-center justify-center gap-2 p-4">
			<MainSearch
				bind:value={searchVal}
				onkeyup={() => {
					queryFiles();
				}}
			/>
			<div>
				<button
					class={`w-30 cursor-pointer rounded-sm border border-amber-300 px-2 py-2 ${showDirSearch ? 'bg-amber-300 text-black' : ''}`}
					onclick={() => {
						showDirSearch = !showDirSearch;
					}}>Add index</button
				>
			</div>
		</div>
		{#if showDirSearch}
			<div class="flex border border-amber-300 p-3">
				<div class="flex flex-col gap-2">
					<DirSearch
						onsubmit={(val) => {
							onSubmitDirSearch(val);
							showDirSearch = false;
						}}
					/>
				</div>
			</div>
		{/if}
	</div>
	{#if indexView}
		<div class="my-3 flex max-w-full items-center justify-between gap-2 border border-blue-400 p-3">
			<p class="wrap-anywhere">Scanning {indexView.queue}</p>
			<div class="flex items-center gap-4">
				<p>{indexView.percent}%</p>
				<button
					class="cursor cursor-pointer border border-red-600 p-1"
					onclick={() => {
						cancelIndex(indexView.id);
					}}>Cancel</button
				>
			</div>
		</div>
	{/if}
	<div class="flex flex-wrap gap-4">
		{#each fileResponse as file (file.id)}
			<FileCard {file} />
		{/each}
	</div>
</div>
