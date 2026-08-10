<script lang="ts">
	import { Tone } from '$lib/types';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Check from '@lucide/svelte/icons/check';
	import Minus from '@lucide/svelte/icons/minus';
	import Archive from '@lucide/svelte/icons/archive';
	type Props = {
		value: 'Published' | 'Draft' | 'Archived';
	};

	let { value }: Props = $props();

	let tone = $derived.by(() => {
		if (value == 'Published') {
			return Tone.Success;
		} else if (value === 'Draft') {
			return Tone.Warning;
		} else {
			return null;
		}
	});
</script>

<Badge {tone}>
	{#snippet icon()}
		{#if tone === Tone.Success}
			<Check />
		{:else if tone === Tone.Warning}
			<Minus />
		{:else}
			<Archive />
		{/if}
	{/snippet}
	{value}
</Badge>
