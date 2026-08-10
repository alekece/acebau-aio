<script lang="ts">
	import { Tooltip } from '@skeletonlabs/skeleton-svelte';
	import { goto } from '$app/navigation';
	import type { Snippet } from 'svelte';

	type Tone = 'surface' | 'tertiary' | 'success' | 'warning' | 'error';

	let {
		href,
		label,
		children,
		tone = 'surface',
		class: className = '',
		onclick
	}: {
		href: string;
		label: string;
		children: Snippet;
		tone?: Tone;
		class?: string;
		onclick?: (event: MouseEvent) => void;
	} = $props();

	let toneClasses = $derived(
		{
			surface: 'bg-transparent !text-surface-600-400 hover:bg-surface-200-800',
			tertiary: 'bg-transparent !text-tertiary-500 hover:bg-tertiary-100-900',
			success: 'bg-transparent !text-success-500 hover:bg-success-100-900',
			warning: 'bg-transparent !text-warning-500 hover:bg-warning-100-900',
			error: 'bg-transparent !text-error-500 hover:bg-error-100-900'
		}[tone]
	);

	function navigate(event: MouseEvent) {
		onclick?.(event);
		if (!event.defaultPrevented) void goto(href);
	}
</script>

<Tooltip openDelay={350} closeDelay={50} positioning={{ placement: 'top' }}>
	<Tooltip.Trigger
		class="btn-icon {toneClasses} focus-visible:ring-2 focus-visible:ring-current focus-visible:outline-none {className}"
		aria-label={label}
		onclick={navigate}
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
