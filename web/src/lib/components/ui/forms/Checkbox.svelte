<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements';
	import { consumeExclusiveGroups, presetGroups, type Size, type Tone } from '../presets';

	type Props = Omit<HTMLInputAttributes, 'checked' | 'class' | 'size' | 'type' | 'value'> & {
		checked?: boolean;
		group?: string[];
		value?: string;
		class?: string;
		size?: Size | `${Size}`;
		small?: boolean;
		medium?: boolean;
		large?: boolean;
		tone?: Tone | `${Tone}`;
		primary?: boolean;
		secondary?: boolean;
		tertiary?: boolean;
		success?: boolean;
		warning?: boolean;
		error?: boolean;
		surface?: boolean;
	};

	let {
		checked = $bindable(false),
		group = $bindable(),
		value,
		class: className = '',
		...props
	}: Props = $props();
	let normalized = $derived(
		consumeExclusiveGroups('Checkbox', props, {
			size: presetGroups.size,
			tone: { ...presetGroups.tone, fallback: 'tertiary' }
		})
	);
	let size = $derived(normalized.normalizedProps.size);
	let tone = $derived(normalized.normalizedProps.tone);
	let rest = $derived(normalized.rest);
	let dimensions = $derived(size === 'sm' ? 'size-4' : size === 'lg' ? 'size-6' : 'size-5');
	let toneClass = $derived(
		({
			brand: 'checked:!bg-primary-500 checked:!accent-primary-500',
			primary: 'checked:!bg-primary-500 checked:!accent-primary-500',
			secondary: 'checked:!bg-secondary-500 checked:!accent-secondary-500',
			tertiary: 'checked:!bg-tertiary-500 checked:!accent-tertiary-500',
			success: 'checked:!bg-success-500 checked:!accent-success-500',
			warning: 'checked:!bg-warning-500 checked:!accent-warning-500',
			error: 'checked:!bg-error-500 checked:!accent-error-500',
			surface: 'checked:!bg-surface-500 checked:!accent-surface-500'
		})[tone ?? 'tertiary']
	);
</script>

{#if group === undefined}
	<input type="checkbox" bind:checked class="checkbox {dimensions} {toneClass} {className}" {...rest} />
{:else}
	<input type="checkbox" bind:group {value} class="checkbox {dimensions} {toneClass} {className}" {...rest} />
{/if}
