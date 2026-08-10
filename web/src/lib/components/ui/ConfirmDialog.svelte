<script lang="ts">
	import type { Snippet } from 'svelte';
	import FormModal from './FormModal.svelte';

	let {
		open = false,
		title,
		description,
		confirmLabel = 'Supprimer',
		busy = false,
		onClose,
		onConfirm,
		children
	}: {
		open?: boolean;
		title: string;
		description?: string;
		confirmLabel?: string;
		busy?: boolean;
		onClose: () => void;
		onConfirm: () => void | Promise<void>;
		children?: Snippet;
	} = $props();
</script>

<FormModal
	{open}
	{title}
	{description}
	submitLabel={confirmLabel}
	{busy}
	destructive
	{onClose}
	onSubmit={onConfirm}
>
	{@render children?.()}
</FormModal>
