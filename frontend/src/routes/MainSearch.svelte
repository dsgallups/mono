<script lang="ts">
	interface Props {
		value: string;
		onkeyup?: () => void;
	}
	let { value = $bindable(), onkeyup }: Props = $props();
</script>

<div>
	<input
		placeholder="Search documents..."
		class="box-border rounded-lg border border-gray-600 bg-gray-800 px-4 py-2 text-gray-200 placeholder-gray-500 focus:border-gray-500 focus:outline-none sm:w-lg lg:w-4xl"
		bind:value
		onkeyup={(e) => {
			if (e.key !== 'Enter') {
				onkeyup?.();
			}
		}}
	/>
</div>
