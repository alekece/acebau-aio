<script lang="ts">
	import Users from '@lucide/svelte/icons/users';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Kpi from '$lib/components/ui/Kpi.svelte';
	import ModuleHeader from '$lib/components/ui/ModuleHeader.svelte';
	import Table from '$lib/components/ui/Table.svelte';
	import DataSourceNotice from '$lib/components/ui/DataSourceNotice.svelte';
	import ResellerEditModal from '$lib/components/resellers/ResellerEditModal.svelte';
	import LinkButton from '$lib/components/ui/LinkButton.svelte';
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();
	let modalOpen = $state(false);
</script>

<svelte:head><title>Revendeurs — Acebau</title></svelte:head>
<div class="mx-auto max-w-[1240px] px-12 py-10 pb-16 max-[850px]:px-5 max-[850px]:py-7">
	<ModuleHeader
		title="Revendeurs"
		description="Suivez les relations commerciales, l’activité et les prochaines relances."
		action="Ajouter un revendeur"
		onAction={() => (modalOpen = true)}
	/><DataSourceNotice visible={data.usingFallback} message={data.loadError} />
	<section class="mb-6 grid grid-cols-4 gap-3.5 max-[1000px]:grid-cols-2 max-[650px]:grid-cols-1">
		<Kpi
			label="Revendeurs actifs"
			value={String(data.kpis.active)}
			detail="Partenaires approuvés"
		/><Kpi
			label="CA revendeurs"
			value={data.kpis.turnover}
			detail="Agrégat temporairement indisponible"
		/><Kpi
			label="Commandes · 6 mois"
			value={data.kpis.orders}
			detail="Agrégat temporairement indisponible"
		/><Kpi
			label="Commande moyenne"
			value={data.kpis.averageOrder}
			detail="Agrégat temporairement indisponible"
		/>
	</section>
	<section class="overflow-hidden card border border-surface-300-700 bg-surface-50-950 shadow-sm">
		<div class="flex items-center justify-between border-b border-surface-300-700 p-5">
			<div>
				<h2 class="m-0 text-lg font-bold text-surface-900-100">Portefeuille revendeurs</h2>
				<p class="mt-1 mb-0 text-sm text-surface-700-300">
					Les suivis datés restent visibles dans la prochaine action.
				</p>
			</div>
			<Users size={20} class="text-tertiary-500" />
		</div>
		<Table
			><thead
				><tr
					><th>Revendeur</th><th>Relation</th><th>Activité</th><th>CA HT</th><th
						>Dernière activité</th
					><th>Prochaine action</th><th></th></tr
				></thead
			><tbody
				>{#each data.resellers as reseller (reseller.id)}<tr
						><td><strong>{reseller.name}</strong><small>{reseller.city}</small></td><td
							><Badge variant="tonal" tone={reseller.tone}>{reseller.state}</Badge></td
						><td>{reseller.activity}</td><td>{reseller.turnover}</td><td>{reseller.last}</td><td
							>{reseller.next}</td
						><td
							><LinkButton
								href={`/resellers/${reseller.id}`}
								variant="outlined"
								tone="surface"
								size="sm">Ouvrir</LinkButton
							></td
						></tr
					>{/each}</tbody
			></Table
		>
	</section>
</div>
<ResellerEditModal open={modalOpen} onClose={() => (modalOpen = false)} />
