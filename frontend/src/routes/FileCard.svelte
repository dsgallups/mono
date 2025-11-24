<script lang="ts">
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

<div class="flex flex-1 shrink flex-col items-center border border-stone-400 p-4">
	<div class="h-10 w-10 bg-red-400"></div>
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
</div>
