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
		let processedPositions = new Set();

		// Sort chunks by their position in the content to process them in order
		let chunkMatches = [];

		for (const chunk of file.chunks || []) {
			if (!chunk.content) continue;

			// Find all occurrences of this chunk in the full text
			let searchText = chunk.content;
			let index = text.indexOf(searchText);

			while (index !== -1) {
				chunkMatches.push({
					start: index,
					end: index + searchText.length,
					chunkId: chunk.id,
					similarity: chunk.similarity
				});
				index = text.indexOf(searchText, index + 1);
			}
		}

		// Sort matches by start position
		chunkMatches.sort((a, b) => a.start - b.start);

		// Remove overlapping matches (keep the first one)
		let filteredMatches = [];
		for (const match of chunkMatches) {
			let overlaps = false;
			for (let pos = match.start; pos < match.end; pos++) {
				if (processedPositions.has(pos)) {
					overlaps = true;
					break;
				}
			}
			if (!overlaps) {
				filteredMatches.push(match);
				for (let pos = match.start; pos < match.end; pos++) {
					processedPositions.add(pos);
				}
			}
		}

		// Build segments
		let lastIndex = 0;
		for (const match of filteredMatches) {
			// Add text before the match (if any)
			if (match.start > lastIndex) {
				segments.push({
					text: text.substring(lastIndex, match.start),
					highlighted: false,
					chunkId: null
				});
			}

			// Add the matched chunk text
			segments.push({
				text: text.substring(match.start, match.end),
				highlighted: true,
				chunkId: match.chunkId,
				similarity: match.similarity
			});

			lastIndex = match.end;
		}

		// Add any remaining text after the last match
		if (lastIndex < text.length) {
			segments.push({
				text: text.substring(lastIndex),
				highlighted: false,
				chunkId: null
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
				<p class="mt-2 text-sm text-blue-600">
					{file.chunks.length} chunk{file.chunks.length === 1 ? '' : 's'} highlighted
				</p>
			{/if}
		</div>

		<div class="prose max-w-none">
			<pre
				class="font-mono text-sm leading-relaxed whitespace-pre-wrap">{#each contentSegments as segment}{#if segment.highlighted}<mark
							class="bg-yellow-200 px-0.5"
							title={`Chunk ID: ${segment.chunkId}, Similarity: ${segment.similarity?.toFixed(3)}`}
							>{segment.text}</mark
						>{:else}{segment.text}{/if}{/each}</pre>
		</div>

		{#if file.chunks && file.chunks.length > 0}
			<div class="mt-8 border-t pt-6">
				<h2 class="mb-4 text-lg font-semibold">Chunks</h2>
				<div class="space-y-4">
					{#each file.chunks as chunk (chunk.id)}
						<div class="rounded border border-gray-200 bg-gray-50 p-3">
							<div class="mb-2 flex items-center justify-between text-sm text-gray-600">
								<span>Chunk ID: {chunk.id}</span>
								<span>Similarity: {chunk.similarity.toFixed(3)}</span>
							</div>
							<p class="text-sm">{chunk.content}</p>
						</div>
					{/each}
				</div>
			</div>
		{/if}
	</div>
{/if}
