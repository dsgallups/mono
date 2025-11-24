<script lang="ts">
	import { page } from '$app/state';
	import type { FileDetails } from '$lib/types';
	import { onMount } from 'svelte';

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

		// Check if there's a chunk to highlight
		if (file.chunks && file.chunks.length > 0 && file.chunks[0].content) {
			const chunk = file.chunks[0];
			const chunkText = chunk.content;

			// Normalize both texts by replacing all consecutive whitespace with single spaces
			let normalizedText = '';
			let normalizedToOriginal = []; // Maps normalized index to original index
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

			// Normalize the chunk text the same way
			let normalizedChunk = chunkText.replace(/\s+/g, ' ').trim();
			normalizedText = normalizedText.trim();

			// Find the chunk in the normalized text
			const normalizedIndex = normalizedText.indexOf(normalizedChunk);
			console.log('normalized index', normalizedIndex);
			console.log('normalized text', normalizedText);
			console.log('chunk', chunkText);

			if (normalizedIndex !== -1) {
				// Map back to original text positions
				const originalStartIndex = normalizedToOriginal[normalizedIndex];
				const normalizedEndIndex = normalizedIndex + normalizedChunk.length - 1;
				const originalEndIndex =
					normalizedEndIndex < normalizedToOriginal.length
						? normalizedToOriginal[normalizedEndIndex] + 1
						: text.length;

				// Add text before the chunk (if any)
				if (originalStartIndex > 0) {
					segments.push({
						text: text.substring(0, originalStartIndex),
						highlighted: false
					});
				}

				// Add the highlighted chunk (from the original text, preserving newlines)
				segments.push({
					text: text.substring(originalStartIndex, originalEndIndex),
					highlighted: true,
					chunkId: chunk.id
				});

				// Add text after the chunk (if any)
				if (originalEndIndex < text.length) {
					segments.push({
						text: text.substring(originalEndIndex),
						highlighted: false
					});
				}
			} else {
				// Chunk not found in content, just show the whole text
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
	<div class="flex h-screen items-center justify-center">
		<p class="text-gray-500">Loading...</p>
	</div>
{:else}
	<div class="container mx-auto max-w-4xl p-6">
		<div class="mb-6 border-b pb-4">
			<h1 class="text-2xl font-bold">{file.title || 'Untitled'}</h1>
			<p class="text-sm text-gray-500">File ID: {file.id}</p>
			{#if file.chunks && file.chunks.length > 0}
				<p class="mt-2 text-sm text-blue-600">Chunk highlighted below</p>
			{/if}
		</div>

		<div class="prose max-w-none">
			<div
				class="overflow-x-auto rounded bg-gray-50 p-4 font-mono text-sm leading-relaxed whitespace-pre-wrap"
			>
				<!--eslint-disable-next-line svelte/require-each-key-->
				{#each contentSegments as segment}
					{#if segment.highlighted}
						<mark class="bg-yellow-200 px-0.5" title={`Chunk ID: ${segment.chunkId}`}
							>{segment.text}</mark
						>
					{:else}
						{segment.text}
					{/if}
				{/each}
			</div>
		</div>

		{#if file.chunks && file.chunks.length > 0 && file.chunks[0]}
			<div class="mt-8 border-t pt-6">
				<h2 class="mb-4 text-lg font-semibold">Highlighted Chunk</h2>
				<div class="rounded border border-gray-200 bg-gray-50 p-3">
					<div class="mb-2 text-sm text-gray-600">
						<span>Chunk ID: {file.chunks[0].id}</span>
					</div>
					<p class="text-sm">{file.chunks[0].content}</p>
				</div>
			</div>
		{/if}
	</div>
{/if}
