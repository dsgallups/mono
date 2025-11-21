<script lang="ts">
	import type { PageProps } from './$types';
	import MainSearch from './MainSearch.svelte';

	let initialSearch = $state(false);

	let { data }: PageProps = $props();

	let searchVal = $state('');
</script>

<div class="flex">
	{#if initialSearch}
		<div class="flex items-center justify-center p-5">
			<MainSearch
				bind:value={searchVal}
				onsubmit={() => {
					initialSearch = false;
				}}
			/>
		</div>
	{:else}
		<div class="flex p-5">
			<MainSearch
				bind:value={searchVal}
				onsubmit={() => {
					initialSearch = false;
				}}
			/>
		</div>
	{/if}

	<div>Data len: {data.files.length}</div>
</div>
