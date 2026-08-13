<script lang="ts">
	import { getMetricDefaults, type MetricKind } from '$lib/settings/metric-defaults';
	import Decimal from 'decimal.js';
	import Badge from '$lib/components/ui/Badge.svelte';
	import { invalidShake } from '$lib/components/ui/animations';
	import { RequiredFeedback } from '$lib/components/ui/presets';

	export type MetricUnit = { value: string; label: string };

	let {
		label,
		value = $bindable('0'),
		unit = $bindable(''),
		units,
		kind,
		defaultUnit,
		hint = '',
		min = '0',
		step = 'any',
		required = false,
		requiredFeedback = RequiredFeedback.None,
		labelVisible = true
	}: {
		label: string;
		value?: string;
		unit?: string;
		units: MetricUnit[];
		kind?: MetricKind;
		defaultUnit?: string;
		hint?: string;
		min?: string | number;
		step?: number | 'any';
		required?: boolean;
		requiredFeedback?: RequiredFeedback;
		labelVisible?: boolean;
	} = $props();

	const metricDefaults = getMetricDefaults();
	let inputElement: HTMLInputElement;
	let touched = $state(false);
	let validationAttempt = $state(0);

	let validationMessage = $derived.by(() => validateDecimal(value, required, min));

	let resolvedUnit = $derived.by(() => {
		const preferred = defaultUnit ?? (kind ? metricDefaults()[kind] : undefined);
		if (units.some((candidate) => candidate.value === unit)) return unit;
		return preferred && units.some((candidate) => candidate.value === preferred)
			? preferred
			: (units[0]?.value ?? '');
	});

	$effect(() => {
		if (unit !== resolvedUnit) unit = resolvedUnit;
	});

	$effect(() => {
		inputElement?.setCustomValidity(validationMessage);
	});

	function validateDecimal(input: string, isRequired: boolean, minimum: string | number) {
		const trimmed = input.trim();
		if (!trimmed) return isRequired ? 'Saisissez une valeur.' : '';

		try {
			if (new Decimal(trimmed).lt(new Decimal(minimum)))
				return `La valeur minimale est ${minimum}.`;
			return '';
		} catch {
			return 'Saisissez un nombre décimal valide.';
		}
	}

	function updateUnit(event: Event) {
		unit = (event.currentTarget as HTMLSelectElement).value;
	}

	function updateValue(event: Event) {
		value = (event.currentTarget as HTMLInputElement).value;
	}
</script>

<label class="label">
	{#if labelVisible}
		<span class="flex items-center justify-between gap-2">
			<span>{label}</span>
			{#if required && requiredFeedback === RequiredFeedback.Full}
				<Badge small tonal surface class="!bg-surface-500/10">Requis</Badge>
			{/if}
		</span>
	{:else}
		<span class="sr-only">{label}</span>
	{/if}
	<div
		use:invalidShake={{
			invalid: Boolean(touched && validationMessage),
			attempt: validationAttempt
		}}
		class={`grid grid-cols-[minmax(0,1fr)_auto] rounded-base shadow-sm focus-within:ring-2 ${touched && validationMessage ? 'ring-2 ring-error-500 focus-within:ring-error-500/30' : 'focus-within:ring-tertiary-500/20'}`}
	>
		<input
			bind:this={inputElement}
			class={`input !rounded-r-none border bg-surface-50-950 px-3 py-2 font-medium text-surface-950-50 shadow-none focus:z-10 focus:ring-0 ${touched && validationMessage ? 'border-error-500 focus:border-error-500' : 'border-surface-300-700 focus:border-tertiary-500'}`}
			type="text"
			inputmode="decimal"
			aria-invalid={touched && validationMessage ? 'true' : undefined}
			{required}
			{value}
			oninput={updateValue}
			onfocus={() => (touched = false)}
			onblur={() => {
				touched = true;
				validationAttempt += 1;
			}}
		/>
		{#if units.length === 1}
			<span
				class={`grid min-w-12 place-items-center !rounded-l-none rounded-r-base border !border-l-0 bg-surface-100-900 px-2.5 py-2 font-medium text-surface-950-50 ${touched && validationMessage ? 'border-error-500' : 'border-surface-300-700'}`}
				aria-label={`Unité pour ${label}`}>{units[0].label}</span
			>
		{:else}
			<select
				class={`select min-w-20 !rounded-l-none border !border-l-0 bg-surface-100-900 py-2 pr-8 pl-2.5 font-medium text-surface-950-50 shadow-none focus:z-10 focus:ring-0 ${touched && validationMessage ? 'border-error-500 focus:border-error-500' : 'border-surface-300-700 focus:border-tertiary-500'}`}
				aria-label={`Unité pour ${label}`}
				value={resolvedUnit}
				onchange={updateUnit}
			>
				{#each units as option (option.value)}<option value={option.value}>{option.label}</option
					>{/each}
			</select>
		{/if}
	</div>
	{#if touched && validationMessage}
		<small class="text-xs font-normal text-error-700-300" role="alert">{validationMessage}</small>
	{/if}
	{#if hint}<small class="text-xs font-normal text-surface-700-300">{hint}</small>{/if}
</label>
