<script lang="ts">
	import FileUp from '@lucide/svelte/icons/file-up';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Kpi from '$lib/components/ui/Kpi.svelte';
	import ModuleHeader from '$lib/components/ui/ModuleHeader.svelte';
	import Table from '$lib/components/ui/Table.svelte';
	import DataSourceNotice from '$lib/components/ui/DataSourceNotice.svelte';
	import InvoiceImportModal from '$lib/components/invoices/InvoiceImportModal.svelte';
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();
	let modalOpen = $state(false);
</script>

<svelte:head><title>Factures — Acebau</title></svelte:head>
<div class="mx-auto max-w-[1240px] px-12 py-10 pb-16 max-[850px]:px-5 max-[850px]:py-7">
	<ModuleHeader
		title="Factures"
		description="Importez, rapprochez et suivez les encaissements de vos deux activités."
		action="Importer une facture"
		onAction={() => (modalOpen = true)}
	/><DataSourceNotice visible={data.usingFallback} message={data.loadError} />
	<section class="mb-6 grid grid-cols-4 gap-3.5 max-[1000px]:grid-cols-2 max-[650px]:grid-cols-1">
		<Kpi
			label="Importées non liées"
			value={String(data.kpis.unlinked)}
			detail="À rattacher à une commande ou activité"
		/><Kpi label="À encaisser" value={String(data.kpis.open)} detail="Factures ouvertes" /><Kpi
			label="En retard"
			value={String(data.kpis.overdue)}
			detail="Relances à préparer"
		/><Kpi label="Échéance sous 7 jours" value="—" detail="Agrégat temporairement indisponible" />
	</section>
	<section class="overflow-hidden card border border-surface-300-700 bg-surface-50-950 shadow-sm">
		<div class="flex items-center justify-between border-b border-surface-300-700 p-5">
			<div>
				<h2 class="m-0 text-lg font-bold text-surface-900-100">Suivi des factures</h2>
				<p class="mt-1 mb-0 text-sm text-surface-700-300">
					Factures importées depuis la plateforme externe.
				</p>
			</div>
			<Button variant="outlined" tone="surface"><FileUp size={16} />Ouvrir un PDF</Button>
		</div>
		<Table
			><thead
				><tr
					><th>Facture</th><th>Client</th><th>Activité</th><th>Émise le</th><th>Échéance</th><th
						>Total</th
					><th>Commande</th><th>État</th><th></th></tr
				></thead
			><tbody
				>{#each data.invoices as invoice (invoice.id)}<tr
						><td><strong>{invoice.no}</strong><small>Métadonnées importées</small></td><td
							>{invoice.customer}</td
						><td>{invoice.activity}</td><td>{invoice.date}</td><td>{invoice.due}</td><td
							>{invoice.total}</td
						><td>{invoice.link}</td><td
							><Badge variant="tonal" tone={invoice.tone}>{invoice.state}</Badge></td
						><td
							><Button variant="outlined" tone="surface" size="sm"
								>{invoice.state === 'En retard' ? 'Relancer' : 'Ouvrir'}</Button
							></td
						></tr
					>{/each}</tbody
			></Table
		>
	</section>
</div>
<InvoiceImportModal open={modalOpen} onClose={() => (modalOpen = false)} />
