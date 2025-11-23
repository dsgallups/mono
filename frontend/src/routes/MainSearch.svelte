<script lang="ts">
	interface Props {
		value: string;
		onsubmit: () => void;
		onkeyup?: () => void;
	}
	let { value = $bindable(), onsubmit, onkeyup }: Props = $props();
</script>

<div>
	<input
		placeholder="search"
		class="text-black sm:w-lg lg:w-4xl"
		bind:value
		onkeyup={(e) => {
			if (e.key === 'Enter') {
				onsubmit();
			} else {
				onkeyup?.();
			}
		}}
	/>
	<button
		onclick={() => {
			onsubmit();
		}}>Go</button
	>
</div>
