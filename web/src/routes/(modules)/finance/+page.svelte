<script lang="ts">
	import ChevronDown from '@lucide/svelte/icons/chevron-down';
	import Button from '$lib/components/ui/Button.svelte';
	import Kpi from '$lib/components/ui/Kpi.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Input from '$lib/components/ui/forms/Input.svelte';
	import ModuleHeader from '$lib/components/ui/ModuleHeader.svelte';
	import Table from '$lib/components/ui/Table.svelte';
	import { Tone } from '$lib/components/ui/presets';
	import DataSourceNotice from '$lib/components/ui/DataSourceNotice.svelte';
	import ExpenseEditModal from '$lib/components/finance/ExpenseEditModal.svelte';
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();
	let period = $state('Mois en cours');
	let month = $state<string | null>(null);
	let modalOpen = $state(false);
</script>

<svelte:head><title>Finance — Acebau</title></svelte:head>
<div class="mx-auto max-w-[1240px] px-12 py-10 pb-16 max-[850px]:px-5 max-[850px]:py-7">
	<ModuleHeader
		title="Finance"
		description="Une lecture prudente de l’activité, de la trésorerie et des déclarations."
		action="Ajouter une dépense"
		onAction={() => (modalOpen = true)}
	/><DataSourceNotice visible={data.usingFallback} message={data.loadError} />
	<div class="mb-6 flex flex-wrap items-center justify-between gap-3">
		<div class="flex gap-2">
			{#each ['Mois en cours', 'Mois précédent', 'Année en cours'] as option (option)}<Button
					variant={period === option ? 'filled' : 'outlined'}
					tone={period === option ? Tone.Tertiary : Tone.Surface}
					size="sm"
					onclick={() => (period = option)}>{option}</Button
				>{/each}
		</div>
		<span class="text-sm text-surface-700-300">Période : août 2026</span>
	</div>
	<section class="mb-6 grid grid-cols-5 gap-3.5 max-[1100px]:grid-cols-3 max-[650px]:grid-cols-1">
		<Kpi
			label="CA HT"
			value={data.summary.turnover}
			detail={`${data.recordCounts.invoices} factures persistées`}
		/><Kpi label="Encaissé" value={data.summary.collected} detail="Agrégat temporaire" /><Kpi
			label="Dépenses"
			value={data.summary.expenses}
			detail={`${data.recordCounts.expenses} écritures persistées`}
		/><Kpi
			label="Résultat avant cotisations"
			value={data.summary.result}
			detail="Agrégat temporaire"
		/><Kpi
			label="Provision TVA / URSSAF"
			value={data.summary.provision}
			detail="Agrégat temporaire"
		/>
	</section>
	<div class="grid gap-6 lg:grid-cols-[1.1fr_0.9fr]">
		<section class="card border border-surface-300-700 bg-surface-50-950 p-6 shadow-sm">
			<h2 class="m-0 text-lg font-bold text-surface-900-100">Trésorerie prudente</h2>
			<p class="mt-1 text-sm text-surface-700-300">Après provisions et factures déjà engagées.</p>
			<strong class="mt-6 block text-4xl font-bold text-surface-900-100">{data.summary.cash}</strong
			>
			<div class="mt-5 h-3 rounded-full bg-surface-200-800">
				<div class="h-3 w-[68%] rounded-full bg-tertiary-500"></div>
			</div>
			<div class="mt-3 flex justify-between text-xs text-surface-700-300">
				<span>Disponible aujourd’hui</span><span>{data.summary.grossCash} brut</span>
			</div>
		</section>
		<section class="card border border-surface-300-700 bg-surface-50-950 p-6 shadow-sm">
			<h2 class="m-0 text-lg font-bold text-surface-900-100">Répartition de l’activité</h2>
			{#each data.summary.activities as item, index (item.name)}<div class="mt-5">
					<div class="mb-1 flex justify-between text-sm">
						<span>{item.name}</span><strong>{item.value}</strong>
					</div>
					<div class="h-2 rounded-full bg-surface-200-800">
						<div
							class="h-2 rounded-full {index === 0 ? 'bg-tertiary-500' : 'bg-secondary-500'}"
							style={`width:${item.percentage}`}
						></div>
					</div>
				</div>{/each}
		</section>
	</div>
	<section
		class="mt-6 overflow-hidden card border border-surface-300-700 bg-surface-50-950 shadow-sm"
	>
		<div class="flex items-center justify-between border-b border-surface-300-700 p-5">
			<div>
				<h2 class="m-0 text-lg font-bold text-surface-900-100">Suivi mensuel</h2>
				<p class="mt-1 mb-0 text-sm text-surface-700-300">Les mois sont fermés par défaut.</p>
			</div>
			<label
				><Input as="select" bind:value={period} aria-label="Activité"
					><option> Toutes les activités </option><option>Acebau</option><option>Freelance</option
					></Input
				></label
			>
		</div>
		<Table
			><thead
				><tr
					><th></th><th>Mois</th><th>CA HT</th><th>Dépenses</th><th>Résultat</th><th
						>TVA collectée</th
					></tr
				></thead
			><tbody
				>{#each data.summary.months as row (row[0])}<tr
						class="cursor-pointer"
						onclick={() => (month = month === row[0] ? null : row[0])}
						><td
							><ChevronDown
								size={17}
								class={month === row[0] ? 'rotate-180 transition' : 'transition'}
							/></td
						><td><strong>{row[0]}</strong></td><td>{row[1]}</td><td>{row[2]}</td><td
							><Badge variant="tonal" tone={Tone.Success}>{row[3]}</Badge></td
						><td>1 196 €</td></tr
					>{#if month === row[0]}<tr class="bg-surface-100-900"
							><td colspan="6"
								><div class="py-2 pl-8 text-sm text-surface-800-200">
									Détail des écritures disponible prochainement.
								</div></td
							></tr
						>{/if}{/each}</tbody
			></Table
		>
	</section>
</div>
<ExpenseEditModal open={modalOpen} onClose={() => (modalOpen = false)} />
