<script lang="ts">
	import { page } from '$app/state';

	interface Props {
		onfocus: () => void;
		onblur: () => void;
		onsubmit: (arg0: string) => void;
	}

	let { onfocus, onblur, onsubmit }: Props = $props();

	let value = $state('..');

	let searchResults = $derived(async () => {
		console.log('here');
		let url = new URL('/api/directories', page.url);
		url.searchParams.set('path', value);
		let response = await fetch(url);
		let responseData: string[] = await response.json();
		return ['..', ...responseData];
	});
</script>

<div class="flex">
	<div class="relative">
		<input
			placeholder="Index Directory"
			class="text-black sm:w-lg lg:w-4xl"
			bind:value
			onkeyup={(e) => {
				if (e.key === 'Enter') {
					onsubmit(value);
				} else {
					if (value === '') {
						onblur();
					} else {
						onfocus();
					}
				}
			}}
		/>

		{#if value !== ''}
			<div class="absolute top-full flex w-full border border-s-slate-950">
				{#await searchResults()}
					<p>Loading</p>
				{:then results}
					<div class="flex max-h-200 flex-1 flex-col gap-2 overflow-auto">
						<!--eslint-disable-next-line svelte/require-each-key-->
						{#each results as result}
							<button
								class="bg-blue-900 text-left hover:bg-blue-800"
								onclick={() => {
									value += `/${result}`;
								}}>{result}</button
							>
						{/each}
					</div>
				{/await}
			</div>
		{/if}
	</div>

	<button
		class="cursor-pointer rounded-sm bg-orange-400"
		onclick={() => {
			onsubmit(value);
		}}
	>
		Do it
	</button>
</div>
