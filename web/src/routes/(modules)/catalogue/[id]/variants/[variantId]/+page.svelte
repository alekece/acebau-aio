<script lang="ts">
	import Badge from '$lib/components/ui/Badge.svelte';
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import Kpi from '$lib/components/ui/Kpi.svelte';
	import TableSection from '$lib/components/ui/TableSection.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<svelte:head><title>{data.variant.displayName} — {data.product.name} — Acebau</title></svelte:head>

<header class="mb-6 flex flex-wrap items-start justify-between gap-4">
	<div>
		<p class="m-0 text-sm font-semibold text-tertiary-700-300">Variante · {data.variant.sku}</p>
		<h2 class="mt-1 mb-0 text-2xl font-bold text-surface-900-100">{data.variant.displayName}</h2>
	</div>
	<Badge variant="tonal" tone={data.variant.status === 'active' ? 'success' : 'surface'}>
		{data.variant.status === 'active' ? 'Active' : 'Archivée'}
	</Badge>
</header>

<section class="mb-6 grid grid-cols-3 gap-4 max-[700px]:grid-cols-1">
	<Kpi label="Disponible" value={data.stock.available} detail="Stock vendable" />
	<Kpi label="À produire" value={data.stock.toProduce} detail="Besoin confirmé" />
	<Kpi label="Défaut" value={data.stock.defective} detail="Sélection manuelle uniquement" />
</section>

<section class="mb-6 grid grid-cols-3 gap-4 max-[700px]:grid-cols-1">
	{#each data.margins as margin (margin.label)}
		<div class="rounded-container border border-surface-300-700 bg-surface-50-950 p-5 shadow-sm">
			<div class="flex items-center justify-between gap-3">
				<strong>{margin.label}</strong><Badge variant="tonal" tone={margin.tone}
					>{margin.value}</Badge
				>
			</div>
		</div>
	{/each}
</section>

<TableSection title="Recette">
	<EmptyState
		compact
		title="Recette à compléter"
		description="Ajoutez des pièces imprimées, des fournitures et le temps de travail manuel."
	/>
</TableSection>
