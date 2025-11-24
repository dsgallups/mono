<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import type { FileDetails } from '$lib/types';
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';

	let file: FileDetails | undefined = $state();

	onMount(async () => {
		const chunkParam = page.url.searchParams.get('chunk');
		const chunkId = chunkParam ? parseInt(chunkParam) : null;

		const url = new URL(`/api/files/${page.params.id}`, page.url);
		if (chunkId !== null) {
			url.searchParams.set('chunk', `${chunkId}`);
		}

		const response = await fetch(url);
		const data: FileDetails = await response.json();
		file = data;
	});

	let contentSegments = $derived.by(() => {
		if (!file || !file.content) return [];

		let segments = [];
		let text = file.content;

		if (file.chunks && file.chunks.length > 0 && file.chunks[0].content) {
			const chunk = file.chunks[0];
			const chunkText = chunk.content;

			let normalizedText = '';
			let normalizedToOriginal = [];
			let inWhitespace = false;

			for (let i = 0; i < text.length; i++) {
				const char = text[i];
				const isWhitespace = /\s/.test(char);

				if (isWhitespace) {
					if (!inWhitespace) {
						normalizedText += ' ';
						normalizedToOriginal.push(i);
						inWhitespace = true;
					}
				} else {
					normalizedText += char;
					normalizedToOriginal.push(i);
					inWhitespace = false;
				}
			}

			let normalizedChunk = chunkText.replace(/\s+/g, ' ').trim();
			normalizedText = normalizedText.trim();

			const normalizedIndex = normalizedText.indexOf(normalizedChunk);
			console.log('normalized index', normalizedIndex);
			console.log('normalized text', normalizedText);
			console.log('chunk', chunkText);

			if (normalizedIndex !== -1) {
				const originalStartIndex = normalizedToOriginal[normalizedIndex];
				const normalizedEndIndex = normalizedIndex + normalizedChunk.length - 1;
				const originalEndIndex =
					normalizedEndIndex < normalizedToOriginal.length
						? normalizedToOriginal[normalizedEndIndex] + 1
						: text.length;

				if (originalStartIndex > 0) {
					segments.push({
						text: text.substring(0, originalStartIndex),
						highlighted: false
					});
				}

				segments.push({
					text: text.substring(originalStartIndex, originalEndIndex),
					highlighted: true,
					chunkId: chunk.id
				});

				if (originalEndIndex < text.length) {
					segments.push({
						text: text.substring(originalEndIndex),
						highlighted: false
					});
				}
			} else {
				segments.push({
					text: text,
					highlighted: false
				});
			}
		} else {
			// No chunks, just show the whole text
			segments.push({
				text: text,
				highlighted: false
			});
		}

		return segments;
	});
</script>

{#if !file}
	<div class="flex h-screen items-center justify-center bg-gray-900">
		<p class="text-gray-400">Loading...</p>
	</div>
{:else}
	<div class="min-h-screen bg-gray-900">
		<div class="container mx-auto max-w-4xl p-6">
			<div class="mb-6 border-b border-gray-700 pb-4">
				<button
					onclick={() => goto(resolve('/'))}
					class="mb-4 flex items-center gap-2 rounded bg-gray-800 px-4 py-2 text-sm font-medium text-gray-300 transition-colors hover:bg-gray-700"
				>
					<svg
						class="h-4 w-4"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						xmlns="http://www.w3.org/2000/svg"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M10 19l-7-7m0 0l7-7m-7 7h18"
						></path>
					</svg>
					Back to Search
				</button>
				<h1 class="text-2xl font-bold text-gray-200">{file.title || 'Untitled'}</h1>
				<p class="text-sm text-gray-400">File ID: {file.id}</p>
				{#if file.chunks.length > 0}
					<button
						class="cursor-pointer text-sm text-blue-600"
						onclick={() => {
							document.location = `/file/${page.params.id}`;
						}}>Clear Highlight</button
					>
				{/if}
			</div>

			<div>
				<div
					class="overflow-x-auto rounded bg-gray-950 p-4 font-mono text-sm leading-relaxed whitespace-pre-wrap text-gray-300"
				>
					<!--eslint-disable-next-line svelte/require-each-key-->
					{#each contentSegments as segment}
						{#if segment.highlighted}
							<mark
								class="bg-yellow-400 px-0.5 font-medium text-black"
								title={`Chunk ID: ${segment.chunkId}`}>{segment.text}</mark
							>
						{:else}
							{segment.text}
						{/if}
					{/each}
				</div>
			</div>
		</div>
	</div>
{/if}
