<script lang="ts">
	import Factory from '@lucide/svelte/icons/factory';
	import Check from '@lucide/svelte/icons/check';
	import ClipboardList from '@lucide/svelte/icons/clipboard-list';
	import ArrowRight from '@lucide/svelte/icons/arrow-right';
	import Button from '$lib/components/ui/Button.svelte';
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import Kpi from '$lib/components/ui/Kpi.svelte';
	import ModuleHeader from '$lib/components/ui/ModuleHeader.svelte';
	import OnboardingPanel from '$lib/components/ui/OnboardingPanel.svelte';
	import PageShell from '$lib/components/ui/PageShell.svelte';
	import Table from '$lib/components/ui/Table.svelte';
	import TableSection from '$lib/components/ui/TableSection.svelte';
	import { Tone } from '$lib/types';
	import DataSourceNotice from '$lib/components/ui/DataSourceNotice.svelte';
	import ProductionEditModal from '$lib/components/production/ProductionEditModal.svelte';
	import ProductionBundleRows from '$lib/components/production/ProductionBundleRows.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	let expanded = $state<string | null>(null);
	let modalOpen = $state(false);
	let productionReady = $derived(data.readiness.every((item) => item.ready));
</script>

<svelte:head><title>Production — Acebau</title></svelte:head>
<PageShell>
	<ModuleHeader
		title="Production"
		action={productionReady ? 'Nouvelle production' : undefined}
		onAction={() => (modalOpen = true)}
	/>
	<DataSourceNotice visible={data.usingFallback} message={data.loadError} />
	{#if productionReady || data.tasks.length > 0}<section
			class="mb-6 grid grid-cols-5 gap-3.5 max-[1100px]:grid-cols-3 max-[650px]:grid-cols-1"
		>
			<Kpi label="À lancer" value={String(data.kpis.toStart)} />
			<Kpi label="Charge à 7 jours" value={data.kpis.load} />
			<Kpi label="Promesses menacées" value={data.kpis.threatened} />
			<Kpi label="Réussite · 30 jours" value={data.kpis.success} />
			<Kpi label="Défauts · 30 jours" value={data.kpis.waste} />
		</section>{/if}
	<div class="grid gap-6">
		{#if !productionReady && data.tasks.length === 0}
			<OnboardingPanel
				title="Préparez votre première production"
				steps={['Configurer l’atelier', 'Créer la production']}
				currentStep={0}
			>
				<div class="grid gap-2">
					{#each data.readiness as item (item.href)}
						<a
							href={item.href}
							class="flex min-h-12 items-center justify-between gap-4 rounded-base border border-surface-300-700 px-4 py-3 hover:bg-surface-100-900"
						>
							<span class="flex items-center gap-3"
								><span
									class="grid size-7 place-items-center rounded-full {item.ready
										? 'bg-success-100-900 text-success-600-400'
										: 'bg-surface-200-800 text-surface-600-400'}"
									>{#if item.ready}<Check size={16} />{:else}<ArrowRight size={16} />{/if}</span
								>{item.label}</span
							>
							<span class="text-sm text-surface-600-400">{item.ready ? 'Prêt' : 'Configurer'}</span>
						</a>
					{/each}
				</div>
			</OnboardingPanel>
		{:else}<TableSection title="Productions">
				{#snippet toolbar()}<Button variant="outlined" tone="surface" size="sm"
						>Voir l’historique</Button
					>{/snippet}
				{#if data.tasks.length === 0}
					<EmptyState
						compact
						title="Aucune production planifiée"
						description="L’atelier est à jour. Créez une production lorsque vous souhaitez fabriquer du stock."
					>
						{#snippet icon()}<ClipboardList size={25} />{/snippet}
						{#snippet actions()}<Button tone="tertiary" onclick={() => (modalOpen = true)}
								>Créer une production</Button
							>{/snippet}
					</EmptyState>
				{:else}<Table responsiveCards>
						<thead
							><tr
								><th></th><th>Opération</th><th>État</th><th>Progression</th><th>Promesse</th><th
								></th></tr
							></thead
						>
						<tbody>
							{#each data.tasks as task (task.id)}
								<ProductionBundleRows
									{task}
									expanded={expanded === task.id}
									pieces={data.pieces}
									filaments={data.filaments}
									machines={data.machines}
									ontoggle={() => (expanded = expanded === task.id ? null : task.id)}
								/>
							{:else}
								<tr
									><td colspan="6" class="py-10 text-center text-surface-700-300"
										>Aucune production planifiée.</td
									></tr
								>
							{/each}
						</tbody>
					</Table>{/if}
			</TableSection>{/if}
		{#if productionReady || data.tasks.length > 0}<div
				class="grid content-start gap-6 lg:grid-cols-2"
			>
				<section class="card border border-surface-300-700 bg-surface-50-950 p-5 shadow-sm">
					<div class="mb-4 flex items-center justify-between">
						<div>
							<h2 class="m-0 text-lg font-bold text-surface-900-100">Charge machines</h2>
							<p class="mt-1 mb-0 text-sm text-surface-700-300">État de charge actuel</p>
						</div>
						<Factory size={20} class="text-tertiary-500" />
					</div>
					{#each data.machineLoad as machine (machine.name)}<div class="mb-4 last:mb-0">
							<div class="mb-1.5 flex justify-between text-sm">
								<span class="font-semibold text-surface-900-100">{machine.name}</span><span
									class="text-surface-700-300">{machine.percentage}</span
								>
							</div>
							<div class="h-2 rounded-full bg-surface-200-800">
								<div
									class="h-2 rounded-full {machine.tone === Tone.Error
										? 'bg-error-500'
										: 'bg-tertiary-500'}"
									style={`width:${machine.percentage}`}
								></div>
							</div>
						</div>{/each}
				</section>
				<section class="card border border-surface-300-700 bg-surface-50-950 p-5 shadow-sm">
					<h2 class="m-0 text-lg font-bold text-surface-900-100">Incidents récents</h2>
					{#if data.incidents.length === 0}<p class="mt-4 mb-0 text-sm text-surface-700-300">
							Aucun incident enregistré.
						</p>{/if}
				</section>
			</div>{/if}
	</div>
</PageShell>
<ProductionEditModal
	open={modalOpen}
	variants={data.variants}
	pieces={data.pieces}
	filaments={data.filaments}
	onClose={() => (modalOpen = false)}
/>
