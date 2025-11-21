<script lang="ts">
	interface Props {
		value: string;
		onsubmit: () => void;
	}
	let { value = $bindable(), onsubmit }: Props = $props();
</script>

<div>
	<input
		placeholder="search"
		class="text-black sm:w-lg lg:w-4xl"
		bind:value
		onkeyup={(e) => {
			if (e.key === 'Enter') {
				onsubmit();
			}
		}}
	/>
	<button
		onclick={() => {
			onsubmit();
		}}>Go</button
	>
</div>
