<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import type { PageProps } from './$types';
	import DirSearch from './DirSearch.svelte';
	import MainSearch from './MainSearch.svelte';

	let initialSearch = $state(true);

	let { data }: PageProps = $props();

	let searchVal = $state('');

	let dirSearchFocused = $state(false);

	function onsubmit() {
		initialSearch = false;

		const url = new URL(page.url);
		url.searchParams.set('q', searchVal);
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		goto(url, { replaceState: true, noScroll: true });
	}

	$inspect('data len: ', data.files.length);
</script>

<div class="flex">
	{#if initialSearch}
		<div class="flex justify-center p-5">
			<div class="flex flex-col gap-2">
				<DirSearch
					onfocus={() => {
						console.log('focusin');
						dirSearchFocused = true;
					}}
					onblur={() => {
						console.log('blurin');
						dirSearchFocused = false;
					}}
					onsubmit={() => {
						console.log('yeah');
					}}
				/>
				{#if !dirSearchFocused}
					<div>
						<MainSearch bind:value={searchVal} {onsubmit} />
					</div>
				{/if}
			</div>
		</div>
	{:else}
		<div class="flex p-5">
			<MainSearch bind:value={searchVal} {onsubmit} />
		</div>
	{/if}
</div>
