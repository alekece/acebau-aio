<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLAttributes } from 'svelte/elements';

	type Variant = 'filled' | 'tonal' | 'outlined';
	type Props = Omit<HTMLAttributes<HTMLSpanElement>, 'class'> & {
		children: Snippet;
		class?: string;
		icon?: Snippet;
		variant?: Variant;
		tone?: string | null;
	};

	let {
		children,
		class: className = '',
		icon,
		variant = 'filled',
		tone,
		...rest
	}: Props = $props();

	let preset = $derived(
		`preset-${variant}${tone ? (variant === 'tonal' ? `-${tone}` : `-${tone}-500`) : ''}`
	);
</script>

<span class="badge [--badge-size:var(--text-sm)] {preset} {className}" {...rest}>
	{#if icon}
		{@render icon()}
		<span>{@render children()}</span>
	{:else}
		{@render children()}
	{/if}
</span>
