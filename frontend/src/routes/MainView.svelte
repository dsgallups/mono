<script lang="ts">
	import { page } from '$app/state';
	import { SvelteURL } from 'svelte/reactivity';
	import { onMount } from 'svelte';
	import MainSearch from './MainSearch.svelte';
	import FileCard from './FileCard.svelte';
	import DirSearch from './DirSearch.svelte';
	import type { FileSimilarity, IndexResponse } from '$lib/types';

	interface Props {
		fetchIndex: () => Promise<IndexResponse[]>;
		onSubmitDirSearch: (path: string) => Promise<void>;
	}

	let { fetchIndex, onSubmitDirSearch }: Props = $props();

	// Search state
	let searchValue = $state('');
	let fileResults: FileSimilarity[] = $state([]);
	let showDirSearch = $state(false);

	// Indexing state
	let currentIndexTask: IndexResponse | null = $state(null);
	let indexPollInterval: number | undefined = $state();
	let pollAttempts = $state(0);

	const MAX_POLL_ATTEMPTS = 5;
	const POLL_INTERVAL_MS = 1000;

	// Derived values
	let indexProgress = $derived.by(() => {
		if (!currentIndexTask) return null;

		return {
			id: currentIndexTask.id,
			path: currentIndexTask.path,
			queue: currentIndexTask.queue,
			percentComplete: (currentIndexTask.progress * 100).toFixed(1)
		};
	});

	// API URL for file queries
	let filesApiUrl = new SvelteURL('/api/files', page.url);

	onMount(async () => {
		await queryFiles();
		await startIndexPolling();
	});

	async function queryFiles() {
		try {
			filesApiUrl.searchParams.set('q', searchValue);
			const response = await fetch(filesApiUrl);

			if (!response.ok) {
				console.error('Failed to fetch files:', response.status);
				fileResults = [];
				return;
			}

			const files: FileSimilarity[] = await response.json();
			fileResults = files;
		} catch (error) {
			console.error('Error querying files:', error);
			fileResults = [];
		}
	}

	async function pollIndexStatus() {
		try {
			const indexTasks = await fetchIndex();

			if (indexTasks.length === 0) {
				currentIndexTask = null;
				return;
			}

			const activeTask = indexTasks.find(
				(task) => task.progress < 1 && task.status === 'in_progress'
			);

			if (activeTask) {
				currentIndexTask = activeTask;
				pollAttempts = 0;
			} else {
				currentIndexTask = null;
				stopIndexPolling();
			}
		} catch (error) {
			console.error('Error polling index status:', error);
			pollAttempts += 1;

			if (pollAttempts >= MAX_POLL_ATTEMPTS) {
				currentIndexTask = null;
				stopIndexPolling();
			}
		}
	}

	async function startIndexPolling() {
		await pollIndexStatus();

		if (indexPollInterval) {
			clearInterval(indexPollInterval);
		}

		indexPollInterval = setInterval(pollIndexStatus, POLL_INTERVAL_MS) as unknown as number;
	}

	function stopIndexPolling() {
		if (indexPollInterval) {
			clearInterval(indexPollInterval);
			indexPollInterval = undefined;
		}
	}

	async function cancelIndexTask(taskId: number) {
		try {
			const url = new URL(`/api/index_tasks/${taskId}`, page.url);
			const response = await fetch(url, { method: 'DELETE' });

			if (response.ok) {
				currentIndexTask = null;
			}
		} catch (error) {
			console.error('Error canceling index task:', error);
		}
	}

	async function handleDirSearchSubmit(path: string) {
		showDirSearch = false;
		await onSubmitDirSearch(path);
		await startIndexPolling();
	}
</script>

<div class="flex flex-col gap-6 p-6">
	<div class="flex flex-col items-center gap-4">
		<div class="flex items-center gap-3">
			<MainSearch bind:value={searchValue} onkeyup={queryFiles} />
			<button
				class="rounded-md border-2 px-4 py-2 font-medium transition-all duration-200 {showDirSearch
					? 'border-amber-500 bg-amber-500 text-white'
					: 'border-amber-500 text-amber-600 hover:bg-amber-50'}"
				onclick={() => (showDirSearch = !showDirSearch)}
			>
				Add Index
			</button>
		</div>

		{#if showDirSearch}
			<div class="w-full max-w-2xl rounded-lg border-2 border-amber-500 bg-amber-50 p-4">
				<DirSearch onsubmit={handleDirSearchSubmit} />
			</div>
		{/if}

		{#if indexProgress}
			<div class="w-full max-w-2xl rounded-lg border-2 border-blue-500 bg-blue-50 p-4">
				<div class="flex items-center justify-between">
					<div class="flex-1">
						<p class="text-sm font-medium text-blue-900">Indexing in progress</p>
						<p class="truncate text-sm text-blue-700">
							Scanning: {indexProgress.queue}
						</p>
					</div>
					<div class="flex items-center gap-3">
						<span class="text-lg font-semibold text-blue-900">
							{indexProgress.percentComplete}%
						</span>
						<button
							class="rounded-md border border-red-500 px-3 py-1 text-sm font-medium text-red-600 transition-colors hover:bg-red-50"
							onclick={() => cancelIndexTask(indexProgress.id)}
						>
							Cancel
						</button>
					</div>
				</div>
			</div>
		{/if}
	</div>

	<div class="flex flex-1 flex-wrap items-start gap-4">
		{#if fileResults.length > 0}
			{#each fileResults as file (file.id)}
				<FileCard {file} search={searchValue} />
			{/each}
		{:else if searchValue}
			<div class="flex w-full flex-col items-center justify-center py-12">
				<svg
					class="mb-4 h-16 w-16 text-gray-400"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
					/>
				</svg>
				<p class="text-lg text-gray-500">No results found for "{searchValue}"</p>
				<p class="mt-1 text-sm text-gray-400">Try a different search term</p>
			</div>
		{:else}
			<div class="flex w-full flex-col items-center justify-center py-12">
				<svg
					class="mb-4 h-16 w-16 text-gray-400"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
					/>
				</svg>
				<p class="text-2xl font-light text-gray-600">Start with a semantic search</p>
				<p class="mt-2 text-sm text-gray-400">Enter keywords to find relevant documents</p>
			</div>
		{/if}
	</div>
</div>
