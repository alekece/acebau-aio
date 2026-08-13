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
			surface:
				'bg-transparent !text-surface-700-300 hover:bg-surface-200-800 hover:!text-surface-950-50 focus-visible:!text-surface-950-50',
			tertiary:
				'bg-transparent !text-surface-700-300 hover:bg-tertiary-100-900 hover:!text-tertiary-600-400 focus-visible:!text-tertiary-600-400',
			success:
				'bg-transparent !text-surface-700-300 hover:bg-success-500/10 hover:!text-success-600-400 focus-visible:bg-success-500/10 focus-visible:!text-success-600-400',
			warning:
				'bg-transparent !text-surface-700-300 hover:bg-warning-100-900 hover:!text-warning-600-400 focus-visible:!text-warning-600-400',
			error:
				'bg-transparent !text-surface-700-300 hover:bg-error-500/10 hover:!text-error-600-400 focus-visible:bg-error-500/10 focus-visible:!text-error-600-400'
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
		data-icon-action
		data-tone={tone}
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

<style>
	:global([data-icon-action][data-tone='error']:hover),
	:global([data-icon-action][data-tone='error']:focus-visible),
	:global([data-icon-action][data-tone='error']:hover svg),
	:global([data-icon-action][data-tone='error']:focus-visible svg) {
		color: var(--color-error-500) !important;
		stroke: var(--color-error-500) !important;
	}

	:global([data-icon-action][data-tone='success']:hover),
	:global([data-icon-action][data-tone='success']:focus-visible),
	:global([data-icon-action][data-tone='success']:hover svg),
	:global([data-icon-action][data-tone='success']:focus-visible svg) {
		color: var(--color-success-500) !important;
		stroke: var(--color-success-500) !important;
	}
</style>
