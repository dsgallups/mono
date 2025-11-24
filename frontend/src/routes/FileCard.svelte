<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import type { FileSimilarity } from '$lib/types';

	interface Props {
		file: FileSimilarity;
		search: string;
	}
	let { file, search }: Props = $props();

	let hasChunks = $derived(file.chunks.length !== 0);

	let chunkContent = $derived.by(() => {
		let content = [];

		for (const chunk of file.chunks) {
			let segments = [];

			if (search && search.length > 0) {
				let text = chunk.content;
				let searchLower = search.toLowerCase();
				let textLower = text.toLowerCase();
				let lastIndex = 0;

				let index = textLower.indexOf(searchLower);
				while (index !== -1) {
					if (index > lastIndex) {
						segments.push({ text: text.substring(lastIndex, index), highlighted: false });
					}

					segments.push({
						text: text.substring(index, index + search.length),
						highlighted: true
					});

					lastIndex = index + search.length;
					index = textLower.indexOf(searchLower, lastIndex);
				}

				if (lastIndex < text.length) {
					segments.push({ text: text.substring(lastIndex), highlighted: false });
				}
			} else {
				segments.push({ text: chunk.content, highlighted: false });
			}

			content.push({
				...chunk,
				segments
			});
		}

		return content;
	});
</script>

<button
	class="flex flex-1 shrink cursor-pointer flex-col items-center border border-stone-400 p-4"
	onclick={() => {
		if (chunkContent.length === 0) {
			goto(resolve(`/file/${file.id}`));
		} else {
			let mostRelevantChunk = chunkContent[0]!;
			goto(resolve(`/file/${file.id}?chunk=${mostRelevantChunk.id}`));
		}
	}}
>
	<div class="flex h-10 w-10 items-center justify-center">
		{#if file.file_type === 'text'}
			<svg
				class="h-16 w-16 text-gray-600"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
				></path>
			</svg>
		{:else if file.file_type === 'jpeg'}
			<svg
				class="h-16 w-16 text-blue-600"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
				></path>
			</svg>
		{:else}
			<svg
				class="h-16 w-16 text-gray-400"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
				></path>
			</svg>
		{/if}
	</div>
	<p>{file.title}</p>
	{#if hasChunks}
		<div class="min-w-100">
			{#each chunkContent as chunk (chunk.id)}
				<div>
					<p>Score: {chunk.similarity}</p>
					<p>
						<!--eslint-disable-next-line svelte/require-each-key-->
						{#each chunk.segments as segment}
							{#if segment.highlighted}
								<mark class="bg-yellow-200">{segment.text}</mark>
							{:else}
								{segment.text}
							{/if}
						{/each}
					</p>
				</div>
				<!-- -->
			{/each}
		</div>
	{/if}
</button>
