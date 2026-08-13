<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLAttributes } from 'svelte/elements';
	import {
		consumeExclusiveGroups,
		presetGroups,
		type Size,
		type Tone,
		type Variant
	} from './presets';

	type Props = Omit<HTMLAttributes<HTMLSpanElement>, 'class'> & {
		children: Snippet;
		class?: string;
		icon?: Snippet;
		size?: Size | `${Size}`;
		small?: boolean;
		medium?: boolean;
		large?: boolean;
		variant?: Variant | `${Variant}`;
		filled?: boolean;
		tonal?: boolean;
		outlined?: boolean;
		tone?: Tone | string | null;
		primary?: boolean;
		secondary?: boolean;
		tertiary?: boolean;
		success?: boolean;
		warning?: boolean;
		error?: boolean;
		surface?: boolean;
	};

	let props: Props = $props();
	let children = $derived(props.children);
	let icon = $derived(props.icon);
	let className = $derived(props.class ?? '');
	let normalized = $derived(consumeExclusiveGroups('Badge', props, presetGroups));
	let size = $derived(normalized.normalizedProps.size);
	let variant = $derived(normalized.normalizedProps.variant);
	let tone = $derived(normalized.normalizedProps.tone);
	let rest = $derived.by(() => {
		const { children: _children, class: _class, icon: _icon, ...attributes } = normalized.rest;
		return attributes;
	});

	let preset = $derived(
		`preset-${variant}${tone ? (variant === 'tonal' ? `-${tone}` : `-${tone}-500`) : ''}`
	);
	let badgeSize = $derived(
		size === 'sm' ? 'var(--text-xs)' : size === 'lg' ? 'var(--text-base)' : 'var(--text-sm)'
	);
</script>

<span class="badge {preset} {className}" style:--badge-size={badgeSize} {...rest}>
	{#if icon}
		{@render icon()}
		<span>{@render children()}</span>
	{:else}
		{@render children()}
	{/if}
</span>
