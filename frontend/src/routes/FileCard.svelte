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

		for (const chunk in file.chunks) {
			//the goal is to highlight characters that match the search query, if there are any.
		}

		return content;
	});
</script>

<div class="flex flex-1 shrink flex-col items-center border border-stone-400 p-4">
	<div class="h-10 w-10 bg-red-400"></div>
	<p>{file.title}</p>
	{#if hasChunks}
		<div class="min-w-100">
			{#each file.chunks as chunk (chunk.id)}
				<div>
					<p>Score: {chunk.similarity}</p>
					<p>{chunk.content}</p>
				</div>
				<!-- -->
			{/each}
		</div>
	{/if}
</div>
