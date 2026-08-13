<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { graphql } from '$lib/api/graphql';
	import MetricInput from '$lib/components/ui/forms/MetricInput.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import DataSourceNotice from '$lib/components/ui/DataSourceNotice.svelte';
	import Input from '$lib/components/ui/forms/Input.svelte';
	import ModuleHeader from '$lib/components/ui/ModuleHeader.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	type SettingsForm = {
		defaultTimeUnit: string;
		defaultMassUnit: string;
		defaultLengthUnit: string;
		defaultPowerUnit: string;
		defaultPageSize: string;
		electricityRate: string;
		electricityUnit: string;
	};
	let settings = $state<SettingsForm>({
		defaultTimeUnit: 'h',
		defaultMassUnit: 'g',
		defaultLengthUnit: 'mm',
		defaultPowerUnit: 'W',
		defaultPageSize: '10',
		electricityRate: '0.25',
		electricityUnit: '€/kWh'
	});
	let initialized = $state(false);
	let savedSnapshot = $state('');
	let saving = $state(false);
	let message = $state('');
	let dirty = $derived(Boolean(savedSnapshot) && JSON.stringify(settings) !== savedSnapshot);
	const electricityUnits = [
		{ value: '€/kWh', label: '€ / kWh' },
		{ value: '€/Wh', label: '€ / Wh' }
	];

	$effect(() => {
		if (initialized) return;
		const source = data.settings;
		const rateMatch = source?.electricityRate.match(/([\d.,]+)\s*(.*)/);
		settings = {
			defaultTimeUnit: source?.defaultTimeUnit ?? 'h',
			defaultMassUnit: source?.defaultMassUnit ?? 'g',
			defaultLengthUnit: source?.defaultLengthUnit ?? 'mm',
			defaultPowerUnit: source?.defaultPowerUnit ?? 'W',
			defaultPageSize: String(source?.defaultPageSize ?? 10),
			electricityRate: rateMatch?.[1].replace(',', '.') ?? '0.25',
			electricityUnit: rateMatch?.[2] || '€/kWh'
		};
		savedSnapshot = JSON.stringify(settings);
		initialized = true;
	});

	async function save() {
		if (!data.settings || data.usingFallback) {
			message = 'L’API doit être disponible pour enregistrer les paramètres.';
			return;
		}
		saving = true;
		message = '';
		try {
			await graphql(
				fetch,
				`
					mutation UpdateSettings($id: String!, $input: ApplicationSettingInput!) {
						updateApplicationSetting(id: $id, input: $input) {
							id
						}
					}
				`,
				{
					id: data.settings.id,
					input: {
						defaultTimeUnit: settings.defaultTimeUnit,
						defaultMassUnit: settings.defaultMassUnit,
						defaultLengthUnit: settings.defaultLengthUnit,
						defaultPowerUnit: settings.defaultPowerUnit,
						defaultPageSize: Number(settings.defaultPageSize),
						electricityRate: `${settings.electricityRate}${settings.electricityUnit}`
					}
				}
			);
			savedSnapshot = JSON.stringify(settings);
			message = 'Paramètres enregistrés.';
			await invalidateAll();
		} catch (cause) {
			message = cause instanceof Error ? cause.message : 'Impossible d’enregistrer les paramètres.';
		} finally {
			saving = false;
		}
	}
</script>

<svelte:head><title>Paramètres — Acebau</title></svelte:head>

<div class="mx-auto max-w-[920px] px-12 py-10 pb-24 max-[850px]:px-5 max-[850px]:py-7">
	<ModuleHeader
		title="Paramètres"
		description="Les valeurs par défaut guident les formulaires sans empêcher une saisie adaptée."
	/>
	<DataSourceNotice visible={data.usingFallback} message={data.loadError} />
	<form
		onsubmit={(event) => {
			event.preventDefault();
			save();
		}}
		class="grid gap-5"
	>
		<section class="grid gap-4 card border border-surface-300-700 bg-surface-50-950 p-6 shadow-sm">
			<div>
				<h2 class="m-0 text-lg font-semibold text-surface-900-100">Atelier</h2>
				<p class="mt-1 mb-0 text-sm text-surface-700-300">
					Le tarif électrique participe au calcul du coût horaire des machines.
				</p>
			</div>
			<MetricInput
				label="Tarif électrique"
				bind:value={settings.electricityRate}
				bind:unit={settings.electricityUnit}
				units={electricityUnits}
				step={0.01}
			/>
		</section>
		<section class="grid gap-4 card border border-surface-300-700 bg-surface-50-950 p-6 shadow-sm">
			<div>
				<h2 class="m-0 text-lg font-semibold text-surface-900-100">Unités par défaut</h2>
				<p class="mt-1 mb-0 text-sm text-surface-700-300">
					Chaque MetricInput utilise la valeur correspondant à sa grandeur lors de sa première
					ouverture.
				</p>
			</div>
			<div class="grid grid-cols-2 gap-4 max-[600px]:grid-cols-1">
				<label class="label"
					>Durée<Input as="select" bind:value={settings.defaultTimeUnit}
						><option value="min">minute</option><option value="h">heure</option><option value="d"
							>jour</option
						><option value="mo">mois</option><option value="y">année</option></Input
					></label
				>
				<label class="label"
					>Masse<Input as="select" bind:value={settings.defaultMassUnit}
						><option value="g">gramme</option><option value="kg">kilogramme</option></Input
					></label
				>
				<label class="label"
					>Longueur<Input as="select" bind:value={settings.defaultLengthUnit}
						><option value="mm">millimètre</option><option value="cm">centimètre</option><option
							value="m">mètre</option
						></Input
					></label
				>
				<label class="label"
					>Puissance<Input as="select" bind:value={settings.defaultPowerUnit}
						><option value="W">watt</option><option value="kW">kilowatt</option></Input
					></label
				>
			</div>
		</section>
		<section class="grid gap-4 card border border-surface-300-700 bg-surface-50-950 p-6 shadow-sm">
			<div>
				<h2 class="m-0 text-lg font-semibold text-surface-900-100">Affichage</h2>
				<p class="mt-1 mb-0 text-sm text-surface-700-300">
					Nombre de lignes affichées par défaut dans les listes.
				</p>
			</div>
			<label class="label max-w-xs"
				>Taille de page<Input as="select" bind:value={settings.defaultPageSize}
					><option value="10">10 lignes</option><option value="25">25 lignes</option><option
						value="50">50 lignes</option
					><option value="100">100 lignes</option></Input
				></label
			>
		</section>
		<div
			class="sticky bottom-4 flex items-center justify-between gap-4 rounded-container border border-surface-300-700 bg-surface-50-950 p-4 shadow-lg"
		>
			<span class="text-sm text-surface-700-300"
				>{dirty ? 'Modifications non enregistrées' : 'Tous les changements sont enregistrés'}</span
			>
			<div class="flex items-center gap-3">
				<span role="status" class="text-sm">{message}</span><Button
					type="submit"
					tone="tertiary"
					disabled={saving || !dirty}>{saving ? 'Enregistrement…' : 'Enregistrer'}</Button
				>
			</div>
		</div>
	</form>
</div>
