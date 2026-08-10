<script lang="ts">
	import type { Snippet } from 'svelte';
	import X from '@lucide/svelte/icons/x';
	import Button from './Button.svelte';
	import Modal from './Modal.svelte';

	let {
		open = false,
		title,
		description = '',
		submitLabel,
		busy = false,
		destructive = false,
		size = 'base',
		onClose,
		onSubmit,
		children
	}: {
		open?: boolean;
		title: string;
		description?: string;
		submitLabel: string;
		busy?: boolean;
		destructive?: boolean;
		size?: 'base' | 'wide';
		onClose: () => void;
		onSubmit: () => void | Promise<void>;
		children?: Snippet;
	} = $props();
</script>

{#if open}
	<Modal
		{open}
		onOpenChange={({ open: nextOpen }) => !nextOpen && onClose()}
		contentClasses={`w-full ${size === 'wide' ? 'max-w-5xl' : 'max-w-xl'} rounded-container border border-surface-300-700 bg-surface-50-950 p-7 shadow-xl max-[600px]:p-5`}
	>
		{#snippet content()}
			<form
				onsubmit={(event) => {
					event.preventDefault();
					onSubmit();
				}}
			>
				<header class="mb-6 flex items-start justify-between gap-4">
					<div>
						<h2 class="m-0 text-2xl font-semibold text-surface-900-100">{title}</h2>
						{#if description}<p class="mt-1 mb-0 text-sm text-surface-700-300">
								{description}
							</p>{/if}
					</div>
					<button
						class="btn-icon preset-tonal-surface"
						type="button"
						aria-label="Fermer"
						onclick={onClose}
					>
						<X size={18} />
					</button>
				</header>
				<div class="grid gap-4">{@render children?.()}</div>
				<footer class="mt-6 flex justify-end gap-2.5 border-t border-surface-300-700 pt-5">
					<Button variant="outlined" tone="secondary" type="button" onclick={onClose}
						>Annuler</Button
					>
					<Button tone={destructive ? 'error' : 'tertiary'} type="submit" disabled={busy}>
						{busy ? 'Enregistrement…' : submitLabel}
					</Button>
				</footer>
			</form>
		{/snippet}
	</Modal>
{/if}
