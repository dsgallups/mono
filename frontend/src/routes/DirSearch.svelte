<script lang="ts">
	import { page } from '$app/state';

	interface Props {
		onfocus?: () => void;
		onblur?: () => void;
		onsubmit: (arg0: string) => void;
	}

	let { onfocus, onblur, onsubmit }: Props = $props();

	let value = $state('');

	let focusSearch = $state(false);

	let queryValue = $derived.by(() => {
		if (focusSearch && value === '') {
			return '.';
		} else {
			return value;
		}
	});

	let searchResults = $derived.by(async () => {
		let url = new URL('/api/directories', page.url);
		url.searchParams.set('path', queryValue);
		let response = await fetch(url);
		let responseData: string[] = await response.json();
		return ['..', ...responseData];
	});
</script>

<div class="flex gap-2">
	<div class="relative flex-1">
		<input
			placeholder="Index Directory"
			class="box-border w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-gray-200 placeholder-gray-500 focus:border-gray-500 focus:outline-none"
			bind:value
			onblur={() => {
				focusSearch = false;
			}}
			onfocus={() => {
				focusSearch = true;
			}}
			onkeyup={(e) => {
				if (e.key === 'Enter') {
					onsubmit(value);
				} else {
					if (value === '') {
						onblur?.();
					} else {
						onfocus?.();
					}
				}
			}}
		/>

		{#if value !== ''}
			<div
				class="absolute top-full z-10 mt-1 w-full rounded-lg border border-gray-700 bg-gray-950 shadow-xl"
			>
				{#await searchResults}
					<p class="p-3 text-gray-400">Loading...</p>
				{:then results}
					<div class="max-h-64 overflow-auto py-1">
						<button
							class="flex w-full items-center gap-2 px-3 py-2 text-left text-gray-300 transition-colors hover:bg-gray-800"
							onclick={() => {
								if (value === '..' || value === '.') {
									value = '.';
								} else {
									//dont need to handle the "" case
									let split = value.split('/');
									split.splice(split.length - 1);
									value = split.join('/');
								}
							}}
						>
							<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M15 19l-7-7 7-7"
								/>
							</svg>
							Back
						</button>
						{#if results.length > 1}
							<div class="mx-3 my-1 border-t border-gray-800"></div>
						{/if}
						<!--eslint-disable-next-line svelte/require-each-key-->
						{#each results as result}
							<button
								class="flex w-full items-center gap-2 px-3 py-2 text-left text-gray-300 transition-colors hover:bg-gray-800"
								onclick={() => {
									let c = value.charAt(value.length - 1);
									if (c === '/') {
										value += result;
									} else {
										value += `/${result}`;
									}
								}}
							>
								<svg
									class="h-4 w-4 text-gray-500"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
									/>
								</svg>
								{result}
							</button>
						{/each}
					</div>
				{/await}
			</div>
		{/if}
	</div>

	<button
		class="rounded-lg bg-amber-600 px-4 py-2 font-medium text-white transition-colors hover:bg-amber-500"
		onclick={() => {
			onsubmit(value);
		}}
	>
		Scan
	</button>
</div>
