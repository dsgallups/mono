<script lang="ts">
	import type { FileSimilarity } from '$lib/types';

	interface Props {
		file: FileSimilarity;
	}
	let { file }: Props = $props();
</script>

<div class="flex shrink border border-stone-400">
	<p>{file.title}</p>
</div>
