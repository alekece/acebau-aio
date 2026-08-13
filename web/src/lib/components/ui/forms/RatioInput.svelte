<script lang="ts">
	import Decimal from 'decimal.js';
	import Badge from '$lib/components/ui/Badge.svelte';
	import { invalidShake } from '$lib/components/ui/animations';
	import { RequiredFeedback } from '$lib/components/ui/presets';

	import type { MetricUnit } from './MetricInput.svelte';

	let {
		label,
		value = $bindable('0'),
		numeratorUnit = $bindable(''),
		denominatorUnit = $bindable(''),
		numeratorUnits,
		denominatorUnits,
		hint = '',
		min = '0',
		required = false,
		requiredFeedback = RequiredFeedback.None
	}: {
		label: string;
		value?: string;
		numeratorUnit?: string;
		denominatorUnit?: string;
		numeratorUnits: MetricUnit[];
		denominatorUnits: MetricUnit[];
		hint?: string;
		min?: string | number;
		required?: boolean;
		requiredFeedback?: RequiredFeedback;
	} = $props();

	let inputElement: HTMLInputElement;
	let touched = $state(false);
	let validationAttempt = $state(0);
	let validationMessage = $derived.by(() => {
		const trimmed = value.trim();
		if (!trimmed) return required ? 'Saisissez une valeur.' : '';
		try {
			if (new Decimal(trimmed).lt(new Decimal(min))) return `La valeur minimale est ${min}.`;
			return '';
		} catch {
			return 'Saisissez un nombre décimal valide.';
		}
	});

	let resolvedNumeratorUnit = $derived(
		numeratorUnits.some((candidate) => candidate.value === numeratorUnit)
			? numeratorUnit
			: (numeratorUnits[0]?.value ?? '')
	);
	let resolvedDenominatorUnit = $derived(
		denominatorUnits.some((candidate) => candidate.value === denominatorUnit)
			? denominatorUnit
			: (denominatorUnits[0]?.value ?? '')
	);

	$effect(() => {
		if (numeratorUnit !== resolvedNumeratorUnit) numeratorUnit = resolvedNumeratorUnit;
		if (denominatorUnit !== resolvedDenominatorUnit) denominatorUnit = resolvedDenominatorUnit;
	});

	$effect(() => inputElement?.setCustomValidity(validationMessage));
</script>

<label class="label">
	<span class="flex items-center justify-between gap-2">
		<span>{label}</span>
		{#if required && requiredFeedback === RequiredFeedback.Full}
			<Badge small tonal surface class="!bg-surface-500/10">Requis</Badge>
		{/if}
	</span>
	<div
		use:invalidShake={{
			invalid: Boolean(touched && validationMessage),
			attempt: validationAttempt
		}}
		class={`flex overflow-hidden rounded-base border bg-surface-50-950 shadow-sm focus-within:ring-2 ${touched && validationMessage ? 'border-error-500 ring-2 ring-error-500 focus-within:border-error-500 focus-within:ring-error-500/30' : 'border-surface-300-700 focus-within:border-tertiary-500 focus-within:ring-tertiary-500/20'}`}
	>
		<input
			bind:this={inputElement}
			class="placeholder:text-surface-500-400 min-w-0 flex-1 border-0 bg-transparent px-3 py-2 font-medium text-surface-950-50 outline-none focus:ring-0"
			type="text"
			inputmode="decimal"
			aria-invalid={touched && validationMessage ? 'true' : undefined}
			{required}
			{value}
			oninput={(event) => {
				value = event.currentTarget.value;
			}}
			onfocus={() => (touched = false)}
			onblur={() => {
				touched = true;
				validationAttempt += 1;
			}}
		/>
		<div
			class="flex shrink-0 items-center gap-1 border-l border-surface-300-700 bg-surface-100-900 px-1.5 text-surface-950-50"
		>
			{#if numeratorUnits.length === 1}
				<span class="px-1.5 py-2 font-medium" aria-label={`Unité du numérateur pour ${label}`}
					>{numeratorUnits[0].label}</span
				>
			{:else}
				<select
					class="select min-w-16 border-0 bg-transparent py-1 pr-7 pl-1.5 font-medium text-surface-950-50 shadow-none hover:bg-surface-200-800 focus:bg-surface-200-800 focus:ring-0"
					aria-label={`Unité du numérateur pour ${label}`}
					value={resolvedNumeratorUnit}
					onchange={(event) => (numeratorUnit = event.currentTarget.value)}
				>
					{#each numeratorUnits as option (option.value)}<option value={option.value}
							>{option.label}</option
						>{/each}
				</select>
			{/if}
			<span class="text-surface-500-400 px-0.5 text-base font-medium" aria-hidden="true">/</span>
			{#if denominatorUnits.length === 1}
				<span class="px-1.5 py-2 font-medium" aria-label={`Unité du dénominateur pour ${label}`}
					>{denominatorUnits[0].label}</span
				>
			{:else}
				<select
					class="select min-w-20 border-0 bg-transparent py-1 pr-7 pl-1.5 font-medium text-surface-950-50 shadow-none hover:bg-surface-200-800 focus:bg-surface-200-800 focus:ring-0"
					aria-label={`Unité du dénominateur pour ${label}`}
					value={resolvedDenominatorUnit}
					onchange={(event) => (denominatorUnit = event.currentTarget.value)}
				>
					{#each denominatorUnits as option (option.value)}<option value={option.value}
							>{option.label}</option
						>{/each}
				</select>
			{/if}
		</div>
	</div>
	{#if touched && validationMessage}
		<small class="text-xs font-normal text-error-700-300" role="alert">{validationMessage}</small>
	{/if}
	{#if hint}<small class="text-xs font-normal text-surface-700-300">{hint}</small>{/if}
</label>
