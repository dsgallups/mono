<script lang="ts">
	import type { FileSimilarity } from '$lib/types';

	interface Props {
		file: FileSimilarity;
	}
	let { file }: Props = $props();

	let hasChunks = $derived(file.chunks.length !== 0);

	//let chunkContent = $derived()
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
