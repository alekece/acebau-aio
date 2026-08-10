<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLAnchorAttributes } from 'svelte/elements';

	type Size = 'sm' | 'base' | 'lg';
	type Variant = 'filled' | 'tonal' | 'outlined';
	type Tone = 'primary' | 'secondary' | 'tertiary' | 'success' | 'warning' | 'error' | 'surface';
	type Props = Omit<HTMLAnchorAttributes, 'class'> & {
		children: Snippet;
		class?: string;
		size?: Size;
		variant?: Variant;
		tone?: Tone;
	};

	let {
		children,
		class: className = '',
		size = 'base',
		variant = 'filled',
		tone,
		onclick,
		...rest
	}: Props = $props();

	let preset = $derived(
		`preset-${variant}${tone ? (variant === 'tonal' ? `-${tone}` : `-${tone}-500`) : ''}`
	);
</script>

<a {onclick} class="btn btn-{size} {preset} {className}" {...rest}>
	{@render children()}
</a>
