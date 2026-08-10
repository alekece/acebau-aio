<script lang="ts">
	import ArrowLeft from '@lucide/svelte/icons/arrow-left';
	import Badge from '$lib/components/ui/Badge.svelte';
	import DataSourceNotice from '$lib/components/ui/DataSourceNotice.svelte';
	import Kpi from '$lib/components/ui/Kpi.svelte';
	import LinkButton from '$lib/components/ui/LinkButton.svelte';
	import Table from '$lib/components/ui/Table.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	const steps = ['Demande', 'Acceptation', 'Production', 'Expédition', 'Facture payée'];
	const stepByState: Record<string, number> = {
		pending: 0,
		accepted: 1,
		in_production: 2,
		ready_to_ship: 3,
		awaiting_payment: 4,
		completed: 4
	};
	let currentStep = $derived(stepByState[data.order.state] ?? 0);
</script>

<svelte:head><title>{data.order.reference} — Commandes — Acebau</title></svelte:head>

<div class="mx-auto max-w-[1120px] px-12 py-10 pb-16 max-[850px]:px-5 max-[850px]:py-7">
	<LinkButton href="/orders" variant="outlined" tone="surface" size="sm" class="mb-6">
		<ArrowLeft size={15} />Commandes
	</LinkButton>
	<DataSourceNotice visible={data.usingFallback} message={data.loadError} />
	<header class="mb-6 flex flex-wrap items-start justify-between gap-4">
		<div>
			<p class="m-0 text-sm font-semibold text-tertiary-700-300">
				{data.order.source} · {data.order.requestedOn}
			</p>
			<h1 class="mt-1 mb-1 text-3xl font-bold text-surface-900-100">{data.order.reference}</h1>
			<p class="m-0 text-surface-700-300">{data.order.customerName}</p>
		</div>
		<Badge variant="tonal" tone={data.order.state === 'pending' ? 'warning' : 'success'}>
			{data.order.progressSummary}
		</Badge>
	</header>
	<ol
		class="mb-6 grid grid-cols-5 overflow-hidden rounded-container border border-surface-300-700 max-[700px]:grid-cols-1"
	>
		{#each steps as step, index (step)}
			<li
				class="flex items-center gap-3 border-r border-surface-300-700 bg-surface-50-950 p-4 last:border-r-0 max-[700px]:border-r-0 max-[700px]:border-b"
			>
				<span
					class="grid size-7 shrink-0 place-items-center rounded-full {index === currentStep
						? 'bg-tertiary-500 text-tertiary-contrast-500'
						: index < currentStep
							? 'bg-success-500 text-success-contrast-500'
							: 'bg-surface-200-800'}">{index + 1}</span
				><span class="text-sm font-semibold">{step}</span>
			</li>
		{/each}
	</ol>
	<section class="mb-6 grid grid-cols-3 gap-4 max-[700px]:grid-cols-1">
		<Kpi label="Total HT" value={data.order.totalHt} detail="Valeur de la demande" />
		<Kpi label="Lignes" value={data.lines.length} detail="Variantes commandées" />
		<Kpi
			label="Prochaine action"
			value={data.order.state === 'pending' ? 'Décider' : 'Suivre'}
			detail={data.order.progressSummary}
		/>
	</section>
	<section class="overflow-hidden card border border-surface-300-700 bg-surface-50-950 shadow-sm">
		<div class="border-b border-surface-300-700 p-5">
			<h2 class="m-0 text-lg font-bold">Contenu</h2>
		</div>
		<Table>
			<thead><tr><th>Variante</th><th>SKU</th><th>Quantité</th><th>Prix unitaire HT</th></tr></thead
			>
			<tbody>
				{#each data.lines as line (line.id)}
					<tr
						><td><strong>{line.variant?.displayName ?? 'Variante indisponible'}</strong></td><td
							>{line.variant?.sku ?? '—'}</td
						><td>{line.quantity}</td><td>{line.unitPriceHt}</td></tr
					>
				{:else}
					<tr
						><td colspan="4" class="py-10 text-center text-surface-700-300"
							>Aucune ligne persistée pour cette commande.</td
						></tr
					>
				{/each}
			</tbody>
		</Table>
	</section>
</div>
