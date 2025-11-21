<script lang="ts">
	let initialSearch = $state(true);

	let searchVal = $state('');
</script>

<div class="flex">
	{#if initialSearch}
		<div class="flex items-center justify-center p-5">
			<input
				placeholder="search"
				class="text-black sm:w-lg lg:w-4xl"
				bind:value={searchVal}
				onkeyup={(e) => {
					if (e.key === 'Enter') {
						initialSearch = false;
					}
				}}
			/>
			<button
				onclick={() => {
					initialSearch = false;
				}}>Go</button
			>
		</div>
	{/if}
</div>
