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
			class="box-border w-full text-black"
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
			<div class="absolute top-full box-border flex w-full border border-blue-600 bg-gray-900">
				{#await searchResults}
					<p>Loading</p>
				{:then results}
					<div class="flex max-h-200 flex-1 flex-col gap-2 overflow-auto">
						<button
							class="bg-blue-900 p-1 text-left hover:bg-blue-800"
							onclick={() => {
								if (value === '..' || value === '.') {
									value = '.';
								} else {
									//dont need to handle the "" case
									let split = value.split('/');
									split.splice(split.length - 1);
									value = split.join('/');
								}
							}}>Back</button
						>
						<!--eslint-disable-next-line svelte/require-each-key-->
						{#each results as result}
							<button
								class="bg-blue-900 p-1 text-left hover:bg-blue-800"
								onclick={() => {
									let c = value.charAt(value.length - 1);
									if (c === '/') {
										value += result;
									} else {
										value += `/${result}`;
									}
								}}>{result}</button
							>
						{/each}
					</div>
				{/await}
			</div>
		{/if}
	</div>

	<button
		class="w-30 cursor-pointer rounded-sm border border-amber-300 px-2 py-2"
		onclick={() => {
			onsubmit(value);
		}}
	>
		Scan
	</button>
</div>
