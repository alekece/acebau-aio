<script lang="ts">
	import ArrowLeft from '@lucide/svelte/icons/arrow-left';
	import Badge from '$lib/components/ui/Badge.svelte';
	import DataSourceNotice from '$lib/components/ui/DataSourceNotice.svelte';
	import Kpi from '$lib/components/ui/Kpi.svelte';
	import LinkButton from '$lib/components/ui/LinkButton.svelte';
	import Table from '$lib/components/ui/Table.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<svelte:head><title>{data.reseller.businessName} — Revendeurs — Acebau</title></svelte:head>

<div class="mx-auto max-w-[1120px] px-12 py-10 pb-16 max-[850px]:px-5 max-[850px]:py-7">
	<LinkButton href="/resellers" variant="outlined" tone="surface" size="sm" class="mb-6">
		<ArrowLeft size={15} />Revendeurs
	</LinkButton>
	<DataSourceNotice visible={data.usingFallback} message={data.loadError} />
	<header class="mb-6 flex flex-wrap items-start justify-between gap-4">
		<div>
			<p class="m-0 text-sm font-semibold text-tertiary-700-300">
				{data.reseller.city} · {data.reseller.country}
			</p>
			<h1 class="mt-1 mb-1 text-3xl font-bold text-surface-900-100">
				{data.reseller.businessName}
			</h1>
			<a
				class="text-sm text-tertiary-700-300 hover:underline"
				href={`mailto:${data.reseller.primaryEmail}`}>{data.reseller.primaryEmail}</a
			>
		</div>
		<Badge variant="tonal" tone={data.reseller.relationship === 'approved' ? 'success' : 'warning'}>
			{data.reseller.relationship === 'approved' ? 'Approuvé' : data.reseller.relationship}
		</Badge>
	</header>
	<section class="mb-6 grid grid-cols-3 gap-4 max-[700px]:grid-cols-1">
		<Kpi label="Commandes" value={data.orders.length} detail="Historique chargé" />
		<Kpi label="Chiffre d’affaires" value="—" detail="Agrégat temporairement indisponible" />
		<Kpi
			label="Prochaine action"
			value={data.reseller.nextAction || 'Aucune'}
			detail={data.reseller.nextActionDate ?? 'Non planifiée'}
		/>
	</section>
	<section class="overflow-hidden card border border-surface-300-700 bg-surface-50-950 shadow-sm">
		<div class="border-b border-surface-300-700 p-5">
			<h2 class="m-0 text-lg font-bold">Commandes</h2>
		</div>
		<Table>
			<thead><tr><th>Référence</th><th>Date</th><th>Total HT</th><th>État</th><th></th></tr></thead>
			<tbody>
				{#each data.orders as order (order.id)}
					<tr
						><td><strong>{order.reference}</strong></td><td>{order.requestedOn}</td><td
							>{order.totalHt}</td
						><td>{order.state}</td><td
							><LinkButton href={`/orders/${order.id}`} variant="outlined" tone="surface" size="sm"
								>Ouvrir</LinkButton
							></td
						></tr
					>
				{:else}
					<tr
						><td colspan="5" class="py-10 text-center text-surface-700-300"
							>Aucune commande associée.</td
						></tr
					>
				{/each}
			</tbody>
		</Table>
	</section>
</div>
