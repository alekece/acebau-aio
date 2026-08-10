<script lang="ts">
	import type { Snippet } from 'svelte';

	type InputElement = 'input' | 'select';

	let {
		as = 'input',
		value = $bindable(''),
		class: className,
		children,
		...rest
	}: {
		as?: InputElement;
		value?: string | number;
		class?: string;
		children?: Snippet;
		[key: string]: unknown;
	} = $props();

	function updateValue(event: Event) {
		value = (event.currentTarget as HTMLInputElement | HTMLSelectElement).value;
	}

	let classes = $derived(
		`${as === 'select' ? 'select' : 'input'} border border-surface-300-700 bg-surface-50-950 px-3 py-2 ${value ? 'font-medium text-surface-950-50' : 'text-surface-700-300'} placeholder:text-surface-500-400 shadow-sm focus:border-tertiary-500 focus:ring-2 focus:ring-tertiary-500/20 ${className ?? ''}`
	);
</script>

{#if as === 'select'}
	<select class={classes} {value} {...rest} oninput={updateValue} onchange={updateValue}>
		{@render children?.()}
	</select>
{:else}
	<input class={classes} {value} {...rest} oninput={updateValue} onchange={updateValue} />
{/if}
