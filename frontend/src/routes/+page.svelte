<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import type { PageProps } from './$types';
	import MainSearch from './MainSearch.svelte';

	let initialSearch = $state(false);

	let { data }: PageProps = $props();

	let searchVal = $state('');

	function onsubmit() {
		initialSearch = false;

		const url = new URL(page.url);
		url.searchParams.set('q', searchVal);
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		goto(url, { replaceState: true, noScroll: true });
	}
</script>

<div class="flex">
	{#if initialSearch}
		<div class="flex items-center justify-center p-5">
			<MainSearch bind:value={searchVal} {onsubmit} />
		</div>
	{:else}
		<div class="flex p-5">
			<MainSearch bind:value={searchVal} {onsubmit} />
		</div>
	{/if}

	<div>Data len: {data.files.length}</div>
</div>
