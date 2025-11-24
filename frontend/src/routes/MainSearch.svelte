<script lang="ts">
	interface Props {
		value: string;
		onkeyup?: () => void;
	}
	let { value = $bindable(), onkeyup }: Props = $props();
</script>

<div>
	<input
		placeholder="search"
		class="text-black sm:w-lg lg:w-4xl"
		bind:value
		onkeyup={(e) => {
			if (e.key !== 'Enter') {
				onkeyup?.();
			}
		}}
	/>
</div>
