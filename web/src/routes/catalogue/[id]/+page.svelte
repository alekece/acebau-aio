<script lang="ts">
	import Badge from '$lib/components/ui/Badge.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import IconLinkAction from '$lib/components/ui/IconLinkAction.svelte';
	import Kpi from '$lib/components/ui/Kpi.svelte';
	import Plus from '@lucide/svelte/icons/plus';
	import Eye from '@lucide/svelte/icons/eye';
	import Table from '$lib/components/ui/Table.svelte';
	import TableSection from '$lib/components/ui/TableSection.svelte';
	import type { PageProps } from './$types';
	import VariantEditModal from '$lib/components/catalogue/VariantEditModal.svelte';

	let { data }: PageProps = $props();
	let variantModalOpen = $state(false);
</script>

<svelte:head><title>{data.product.name} — Catalogue — Acebau</title></svelte:head>

<section class="mb-6 grid grid-cols-3 gap-4 max-[700px]:grid-cols-1">
	<Kpi label="Variantes" value={data.variants.length} detail="Configurations persistées" />
	<Kpi
		label="Personnalisable"
		value={data.product.customizable ? 'Oui' : 'Non'}
		detail="Catalogue revendeur"
	/>
	<Kpi label="Stock" value="—" detail="Agrégat temporairement indisponible" />
</section>
<TableSection id="variantes" title="Variantes">
	{#snippet toolbar()}<Button tone="tertiary" size="sm" onclick={() => (variantModalOpen = true)}
			><Plus size={16} />Nouvelle variante</Button
		>{/snippet}
	{#if data.variants.length === 0}
		<EmptyState
			compact
			title="Aucune variante"
			description="Ajoutez une variante vide ou copiez une variante de ce produit."
		>
			{#snippet actions()}<Button tone="tertiary" onclick={() => (variantModalOpen = true)}
					><Plus size={16} />Créer une variante</Button
				>{/snippet}
		</EmptyState>
	{:else}<Table responsiveCards>
			<thead>
				<tr
					><th>Variante</th><th>SKU</th><th>Prix public TTC</th><th>Prix revendeur HT</th><th
						>État</th
					><th></th></tr
				>
			</thead>
			<tbody>
				{#each data.variants as variant (variant.id)}
					<tr>
						<td data-label="Variante"><strong>{variant.displayName}</strong></td><td
							data-label="SKU">{variant.sku}</td
						><td data-label="Prix public TTC">{variant.retailPrice}</td><td
							data-label="Prix revendeur HT">{variant.resellerPrice}</td
						><td data-label="État"
							><Badge variant="tonal" tone={variant.status === 'active' ? 'success' : 'surface'}
								>{variant.status === 'active' ? 'Active' : 'Archivée'}</Badge
							></td
						><td data-label="" class="mobile-card-actions"
							><IconLinkAction
								href={`/catalogue/${data.product.id}/variants/${variant.id}`}
								label={`Ouvrir ${variant.displayName}`}><Eye size={18} /></IconLinkAction
							></td
						>
					</tr>
				{/each}
			</tbody>
		</Table>
	{/if}
</TableSection>

<VariantEditModal
	open={variantModalOpen}
	product={data.product}
	variants={data.variants}
	onClose={() => (variantModalOpen = false)}
/>
