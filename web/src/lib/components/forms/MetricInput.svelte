<script lang="ts">
	import { getMetricDefaults, type MetricKind } from '$lib/settings/metric-defaults';
	import Decimal from 'decimal.js';

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
		required = false
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
	} = $props();

	const metricDefaults = getMetricDefaults();
	let inputElement: HTMLInputElement;
	let touched = $state(false);

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
			if (new Decimal(trimmed).lt(new Decimal(minimum))) return `La valeur minimale est ${minimum}.`;
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
		touched = true;
	}
</script>

<label class="label">
	<span>{label}</span>
	<div
		class="grid grid-cols-[minmax(0,1fr)_auto] rounded-base shadow-sm focus-within:ring-2 focus-within:ring-tertiary-500/20"
	>
		<input
			bind:this={inputElement}
			class="input min-h-[42px] !rounded-r-none border border-surface-300-700 bg-surface-50-950 px-3 py-2 font-medium text-surface-950-50 shadow-none focus:z-10 focus:border-tertiary-500 focus:ring-0"
			type="text"
			inputmode="decimal"
			aria-invalid={validationMessage ? 'true' : undefined}
			{required}
			value={value}
			oninput={updateValue}
			onblur={() => (touched = true)}
		/>
		{#if units.length === 1}
			<span
				class="grid min-h-[42px] min-w-12 place-items-center !rounded-l-none rounded-r-base border !border-l-0 border-surface-300-700 bg-surface-100-900 px-2.5 py-2 font-medium text-surface-950-50"
				aria-label={`Unité pour ${label}`}>{units[0].label}</span
			>
		{:else}
			<select
				class="select min-h-[42px] min-w-20 !rounded-l-none border !border-l-0 border-surface-300-700 bg-surface-100-900 py-2 pr-8 pl-2.5 font-medium text-surface-950-50 shadow-none focus:z-10 focus:border-tertiary-500 focus:ring-0"
				aria-label={`Unité pour ${label}`}
				value={resolvedUnit}
				onchange={updateUnit}
			>
				{#each units as option (option.value)}<option value={option.value}
					>{option.label}</option
				>{/each}
			</select>
		{/if}
	</div>
	{#if touched && validationMessage}
		<small class="text-xs font-normal text-error-700-300" role="alert"
			>{validationMessage}</small
		>
	{/if}
	{#if hint}<small class="text-xs font-normal text-surface-700-300">{hint}</small>{/if}
</label>
