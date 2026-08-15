<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';
	import { presetClass, Shade, Size, Tone, Variant } from './presets';
	type Props = Omit<HTMLButtonAttributes, 'class'> & {
		children: Snippet;
		class?: string;
		size?: Size | `${Size}`;
		variant?: Variant | `${Variant}`;
		tone?: Tone | `${Tone}`;
		shade?: Shade | `${Shade}`;
	};

	let {
		children,
		class: className = '',
		size = Size.Medium,
		variant = Variant.Filled,
		tone,
		shade = Shade.Default,
		type = 'button',
		onclick,
		...rest
	}: Props = $props();

	let preset = $derived(presetClass(variant, tone, shade));
</script>

<button {type} {onclick} class="btn btn-{size} {preset} {className}" {...rest}>
	{@render children()}
</button>
