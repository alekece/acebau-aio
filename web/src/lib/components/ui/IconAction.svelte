<script lang="ts">
	import { Tooltip } from '@skeletonlabs/skeleton-svelte';
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';

	type Tone = 'surface' | 'tertiary' | 'success' | 'warning' | 'error';
	type Props = Omit<HTMLButtonAttributes, 'children' | 'class' | 'title' | 'disabled'> & {
		children: Snippet;
		label: string;
		class?: string;
		disabled?: boolean;
		tone?: Tone;
	};

	let {
		children,
		label,
		class: className = '',
		disabled = false,
		tone = 'surface',
		onclick,
		type = 'button',
		...rest
	}: Props = $props();

	let toneClasses = $derived(
		{
			surface: 'bg-transparent !text-surface-600-400 hover:bg-surface-200-800',
			tertiary: 'bg-transparent !text-tertiary-500 hover:bg-tertiary-100-900',
			success: 'bg-transparent !text-success-500 hover:bg-success-100-900',
			warning: 'bg-transparent !text-warning-500 hover:bg-warning-100-900',
			error: 'bg-transparent !text-error-500 hover:bg-error-100-900'
		}[tone]
	);

	function handleClick(event: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement }) {
		if (disabled) {
			event.preventDefault();
			return;
		}
		onclick?.(event);
	}
</script>

<Tooltip openDelay={350} closeDelay={50} positioning={{ placement: 'top' }}>
	<Tooltip.Trigger
		{...rest}
		{type}
		class="btn-icon {toneClasses} focus-visible:ring-2 focus-visible:ring-current focus-visible:outline-none {disabled
			? 'cursor-not-allowed opacity-45'
			: ''} {className}"
		aria-label={label}
		aria-disabled={disabled}
		onclick={handleClick}
	>
		{@render children()}
	</Tooltip.Trigger>
	<Tooltip.Positioner class="z-[100]">
		<Tooltip.Content
			class="rounded-base bg-surface-950 px-2.5 py-1.5 text-xs font-medium text-surface-50 shadow-lg dark:bg-surface-50 dark:text-surface-950"
		>
			{label}
		</Tooltip.Content>
	</Tooltip.Positioner>
</Tooltip>
