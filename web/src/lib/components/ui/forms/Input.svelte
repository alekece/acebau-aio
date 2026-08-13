<script lang="ts">
	import type { Snippet } from 'svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import { RequiredFeedback } from '$lib/components/ui/presets';

	type InputElement = 'input' | 'select';

	let {
		as = 'input',
		value = $bindable(''),
		class: className,
		children,
		requiredFeedback = RequiredFeedback.Full,
		invalid = false,
		...rest
	}: {
		as?: InputElement;
		value?: string | number;
		class?: string;
		children?: Snippet;
		requiredFeedback?: RequiredFeedback;
		invalid?: boolean;
		[key: string]: unknown;
	} = $props();

	function updateValue(event: Event) {
		value = (event.currentTarget as HTMLInputElement | HTMLSelectElement).value;
	}

	let classes = $derived(
		`${as === 'select' ? 'select' : 'input'} border bg-surface-50-950 px-3 py-2 ${value ? 'font-medium text-surface-950-50' : 'font-light text-surface-500-400'} placeholder:font-light placeholder:italic placeholder:text-surface-500-400 placeholder:opacity-50 shadow-sm focus:ring-2 ${invalid ? 'border-error-500 focus:border-error-500 focus:ring-error-500/20' : 'border-surface-300-700 focus:border-tertiary-500 focus:ring-tertiary-500/20'} ${className ?? ''}`
	);
</script>

{#if as === 'select'}
	{#if requiredFeedback === RequiredFeedback.Full && rest.required}
		<Badge small tonal surface class="absolute top-0 right-0 !bg-surface-500/10" data-required-badge
			>Requis</Badge
		>
	{/if}
	<select
		class={classes}
		{value}
		aria-invalid={invalid ? 'true' : undefined}
		{...rest}
		oninput={updateValue}
		onchange={updateValue}
	>
		{@render children?.()}
	</select>
{:else}
	{#if requiredFeedback === RequiredFeedback.Full && rest.required}
		<Badge small tonal surface class="absolute top-0 right-0 !bg-surface-500/10" data-required-badge
			>Requis</Badge
		>
	{/if}
	<input
		class={classes}
		{value}
		aria-invalid={invalid ? 'true' : undefined}
		{...rest}
		oninput={updateValue}
		onchange={updateValue}
	/>
{/if}
