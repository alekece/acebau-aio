<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Dialog } from '@skeletonlabs/skeleton-svelte';

	let {
		open = false,
		onOpenChange,
		contentClasses = '',
		content
	}: {
		open?: boolean;
		onOpenChange?: (details: { open: boolean }) => void;
		contentClasses?: string;
		content?: Snippet;
	} = $props();
</script>

<Dialog {open} onOpenChange={(details) => onOpenChange?.(details)}>
	<Dialog.Backdrop class="fixed inset-0 z-40 bg-black/50" />
	<Dialog.Positioner class="fixed inset-0 z-50 grid place-items-center p-4">
		<Dialog.Content class={contentClasses}>
			{@render content?.()}
		</Dialog.Content>
	</Dialog.Positioner>
</Dialog>
