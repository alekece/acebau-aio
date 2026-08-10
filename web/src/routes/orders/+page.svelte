<script lang="ts">
	import ChevronDown from '@lucide/svelte/icons/chevron-down';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Kpi from '$lib/components/ui/Kpi.svelte';
	import ModuleHeader from '$lib/components/ui/ModuleHeader.svelte';
	import Table from '$lib/components/ui/Table.svelte';
	import DataSourceNotice from '$lib/components/ui/DataSourceNotice.svelte';
	import OrderEditModal from '$lib/components/orders/OrderEditModal.svelte';
	import LinkButton from '$lib/components/ui/LinkButton.svelte';
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();
	let open = $state<string | null>(null);
	let modalOpen = $state(false);
</script>

<svelte:head><title>Commandes — Acebau</title></svelte:head>
<div class="mx-auto max-w-[1240px] px-12 py-10 pb-16 max-[850px]:px-5 max-[850px]:py-7">
	<ModuleHeader
		title="Commandes"
		description="Traitez les demandes, suivez la production et préparez les expéditions."
		action="Saisir une commande"
		onAction={() => (modalOpen = true)}
	/><DataSourceNotice visible={data.usingFallback} message={data.loadError} />
	<section class="mb-6 grid grid-cols-4 gap-3.5 max-[1000px]:grid-cols-2 max-[650px]:grid-cols-1">
		<Kpi
			label="Commandes ouvertes"
			value={String(data.orders.length)}
			detail="Tous états opérationnels"
		/><Kpi
			label="Acceptation → expédition"
			value="—"
			detail="Agrégat temporairement indisponible"
		/><Kpi
			label="Livraisons à l’heure"
			value="—"
			detail="Agrégat temporairement indisponible"
		/><Kpi
			label="Demandes à chiffrer"
			value={String(data.lifecycle[0].count)}
			detail="Réponse attendue"
		/>
	</section>
	<section class="mb-6 grid grid-cols-5 gap-3 max-[800px]:grid-cols-2 max-[500px]:grid-cols-1">
		{#each data.lifecycle as item (item.label)}<div
				class="rounded-base border border-surface-300-700 bg-surface-50-950 p-4"
			>
				<span class="text-xs text-surface-700-300">{item.label}</span>
				<div class="mt-2 flex items-center gap-2">
					<strong class="text-xl text-surface-900-100">{item.count}</strong><Badge
						variant="tonal"
						tone={item.tone}>{item.label}</Badge
					>
				</div>
			</div>{/each}
	</section>
	<section class="overflow-hidden card border border-surface-300-700 bg-surface-50-950 shadow-sm">
		<div class="border-b border-surface-300-700 p-5">
			<h2 class="m-0 text-lg font-bold text-surface-900-100">Toutes les commandes</h2>
		</div>
		<Table
			><thead
				><tr
					><th></th><th>Commande</th><th>Source</th><th>Date</th><th>Valeur</th><th>État</th><th
						>Progression</th
					><th></th></tr
				></thead
			><tbody
				>{#each data.orders as order (order.id)}<tr
						class="cursor-pointer"
						onclick={() => (open = open === order.id ? null : order.id)}
						><td
							><ChevronDown
								size={17}
								class={open === order.id ? 'rotate-180 transition' : 'transition'}
							/></td
						><td><strong>{order.id} · {order.customer}</strong><small>{order.customer}</small></td
						><td>{order.source}</td><td>{order.date}</td><td>{order.value}</td><td
							><Badge variant="tonal" tone={order.tone}>{order.status}</Badge></td
						><td>{order.progress}</td><td
							><LinkButton
								href={`/orders/${order.recordId}`}
								variant="outlined"
								tone="surface"
								size="sm"
								onclick={(event) => event.stopPropagation()}>{order.action}</LinkButton
							></td
						></tr
					>{#if open === order.id}<tr class="bg-surface-100-900"
							><td colspan="8"
								><div class="py-2 pl-8 text-sm text-surface-800-200">{order.lines}</div></td
							></tr
						>{/if}{/each}</tbody
			></Table
		>
	</section>
</div>
<OrderEditModal open={modalOpen} onClose={() => (modalOpen = false)} />
