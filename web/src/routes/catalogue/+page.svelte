<script lang="ts">
	import ChevronDown from '@lucide/svelte/icons/chevron-down';
	import Check from '@lucide/svelte/icons/check';
	import Eye from '@lucide/svelte/icons/eye';
	import Pencil from '@lucide/svelte/icons/pencil';
	import Trash2 from '@lucide/svelte/icons/trash-2';
	import X from '@lucide/svelte/icons/x';
	import PackagePlus from '@lucide/svelte/icons/package-plus';
	import Search from '@lucide/svelte/icons/search';
	import { SvelteSet } from 'svelte/reactivity';
	import { invalidateAll } from '$app/navigation';
	import { graphql } from '$lib/api/graphql';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import ConfirmDialog from '$lib/components/ui/ConfirmDialog.svelte';
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import IconAction from '$lib/components/ui/IconAction.svelte';
	import IconLinkAction from '$lib/components/ui/IconLinkAction.svelte';
	import Kpi from '$lib/components/ui/Kpi.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import ModuleHeader from '$lib/components/ui/ModuleHeader.svelte';
	import OnboardingPanel from '$lib/components/ui/OnboardingPanel.svelte';
	import PageShell from '$lib/components/ui/PageShell.svelte';
	import Table from '$lib/components/ui/Table.svelte';
	import TableSection from '$lib/components/ui/TableSection.svelte';
	import DataSourceNotice from '$lib/components/ui/DataSourceNotice.svelte';
	import ProductEditModal from '$lib/components/catalogue/ProductEditModal.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	let query = $state('');
	const expandedProducts = new SvelteSet<string>();
	let editOpen = $state(false);
	let edits = $state<
		Record<
			string,
			{
				name: string;
				collection: string;
				category: string;
				shortDescription: string;
				customizable: boolean;
				busy: boolean;
				error: string;
			}
		>
	>({});
	let deleteTarget = $state<(typeof data.products)[number] | null>(null);
	let deleteBusy = $state(false);
	let filtered = $derived(
		data.products.filter((product) =>
			`${product.name} ${product.category}`.toLowerCase().includes(query.toLowerCase())
		)
	);

	function toggleProduct(id: string) {
		if (expandedProducts.has(id)) expandedProducts.delete(id);
		else expandedProducts.add(id);
	}

	function beginEdit(product: (typeof data.products)[number]) {
		edits = {
			...edits,
			[product.id]: {
				name: product.name,
				collection: product.collection,
				category: product.categoryName,
				shortDescription: product.shortDescription,
				customizable: product.customizable,
				busy: false,
				error: ''
			}
		};
	}

	function cancelEdit(id: string) {
		const remaining = { ...edits };
		delete remaining[id];
		edits = remaining;
	}

	async function saveEdit(id: string) {
		const edit = edits[id];
		if (!edit?.name.trim() || !edit.collection.trim() || !edit.category.trim()) return;
		edits = { ...edits, [id]: { ...edit, busy: true, error: '' } };
		try {
			await graphql(
				fetch,
				`
					mutation UpdateProduct($id: String!, $input: ProductInput!) {
						updateProduct(id: $id, input: $input) {
							id
						}
					}
				`,
				{
					id,
					input: {
						name: edit.name.trim(),
						collection: edit.collection.trim(),
						category: edit.category.trim(),
						shortDescription: edit.shortDescription.trim(),
						customizable: edit.customizable
					}
				}
			);
			cancelEdit(id);
			await invalidateAll();
		} catch (cause) {
			edits = {
				...edits,
				[id]: {
					...edit,
					busy: false,
					error: cause instanceof Error ? cause.message : 'Impossible de modifier le produit.'
				}
			};
		}
	}

	async function deleteProduct() {
		if (!deleteTarget) return;
		deleteBusy = true;
		try {
			await graphql(
				fetch,
				`
					mutation DeleteProduct($id: String!) {
						deleteProduct(id: $id)
					}
				`,
				{ id: deleteTarget.id }
			);
			deleteTarget = null;
			await invalidateAll();
		} finally {
			deleteBusy = false;
		}
	}
</script>

<svelte:head><title>Catalogue — Acebau</title></svelte:head>
<PageShell>
	<ModuleHeader title="Catalogue" action="Nouveau produit" onAction={() => (editOpen = true)} />
	<DataSourceNotice visible={data.usingFallback} message={data.loadError} />
	{#if data.products.length === 0}
		<OnboardingPanel
			title="Configurez votre catalogue"
			steps={['Créer un produit', 'Ajouter une variante', 'Compléter la recette']}
			currentStep={0}
		>
			<EmptyState
				compact
				title="Créez votre premier produit"
				description="Commencez par son identité. La première variante reste facultative."
			>
				{#snippet icon()}<PackagePlus size={25} />{/snippet}
				{#snippet actions()}<Button tone="tertiary" onclick={() => (editOpen = true)}
						>Créer un produit</Button
					>{/snippet}
			</EmptyState>
		</OnboardingPanel>
	{:else}
		<section class="mb-6 grid max-w-3xl grid-cols-3 gap-3.5 max-[650px]:grid-cols-1">
			<Kpi label="Produits actifs" value={String(data.kpis.activeProducts)} /><Kpi
				label="Variantes actives"
				value={String(data.kpis.activeVariants)}
			/><Kpi label="Sous la marge cible" value={String(data.kpis.belowMargin)} />
		</section>
		<TableSection title="Produits">
			{#snippet toolbar()}
				<label class="relative"
					><Search
						size={16}
						class="text-surface-500-400 pointer-events-none absolute top-1/2 left-3 -translate-y-1/2"
					/><Input bind:value={query} placeholder="Rechercher un produit" class="pl-9" /></label
				>
			{/snippet}
			<Table responsiveCards
				><thead
					><tr
						><th></th><th>Produit</th><th>Variantes</th><th>Prix</th><th>Ventes</th><th>État</th><th
							>Alertes</th
						><th></th></tr
					></thead
				><tbody
					>{#each filtered as product (product.id)}<tr
							class="cursor-pointer"
							onclick={() => toggleProduct(product.id)}
							><td data-label="" class="mobile-card-hidden"
								><ChevronDown
									size={17}
									class={expandedProducts.has(product.id) ? 'rotate-180 transition' : 'transition'}
								/></td
							>{#if edits[product.id]}
								<td data-label="Produit" onclick={(event) => event.stopPropagation()}>
									<div class="grid gap-2">
										<Input bind:value={edits[product.id].name} aria-label="Nom du produit" />
										<div class="grid grid-cols-2 gap-2">
											<Input bind:value={edits[product.id].category} aria-label="Catégorie" />
											<Input bind:value={edits[product.id].collection} aria-label="Collection" />
										</div>
										{#if edits[product.id].error}<small class="text-error-600-400" role="alert"
												>{edits[product.id].error}</small
											>{/if}
									</div>
								</td>
							{:else}<td data-label="Produit"
									><strong>{product.name}</strong><small>{product.category}</small></td
								>{/if}<td data-label="Variantes">{product.variants}</td><td data-label="Prix"
								>{product.price}</td
							><td data-label="Ventes">{product.sales}</td><td data-label="État"
								><Badge variant="tonal" tone={product.tone}>{product.status}</Badge></td
							><td data-label="Alertes">{product.alert}</td><td
								data-label=""
								class="mobile-card-actions"
								onclick={(event) => event.stopPropagation()}
							>
								{#if edits[product.id]}
									<IconAction
										label={`Confirmer la modification de ${product.name}`}
										tone="success"
										disabled={edits[product.id].busy}
										onclick={() => saveEdit(product.id)}><Check size={18} /></IconAction
									>
									<IconAction
										label={`Annuler la modification de ${product.name}`}
										tone="error"
										disabled={edits[product.id].busy}
										onclick={() => cancelEdit(product.id)}><X size={18} /></IconAction
									>
								{:else}
									<IconLinkAction
										href={`/catalogue/${product.id}`}
										label={`Ouvrir ${product.name}`}
										onclick={(event) => event.stopPropagation()}><Eye size={18} /></IconLinkAction
									>
									<IconAction label={`Modifier ${product.name}`} onclick={() => beginEdit(product)}
										><Pencil size={18} /></IconAction
									>
									<IconAction
										label={`Supprimer ${product.name}`}
										tone="error"
										onclick={() => (deleteTarget = product)}><Trash2 size={18} /></IconAction
									>
								{/if}
							</td></tr
						>{#if expandedProducts.has(product.id)}<tr class="bg-surface-100-900"
								><td colspan="8" data-label="Variantes"
									><div class="py-1 pl-8">
										<div
											class="mb-2 grid grid-cols-4 gap-3 text-xs font-semibold tracking-wide text-surface-700-300 uppercase"
										>
											<span>Variante</span><span>SKU</span><span>Prix</span><span>Marge site</span>
										</div>
										{#each product.children as variant (variant.id)}<a
												href={`/catalogue/${product.id}/variants/${variant.id}`}
												class="grid grid-cols-4 gap-3 border-l-2 border-tertiary-500/50 px-3 py-2 text-sm text-surface-800-200"
											>
												<span>{variant.name}</span><span>{variant.sku}</span><span
													>{variant.price}</span
												><span>{variant.margin}</span>
											</a>{/each}
									</div></td
								></tr
							>{/if}{:else}<tr
							><td data-label="" colspan="8" class="py-9 text-center text-surface-700-300"
								>Aucun produit ne correspond à cette recherche.</td
							></tr
						>{/each}</tbody
				></Table
			>
		</TableSection>
		<section class="mt-6 grid grid-cols-3 gap-4 max-[800px]:grid-cols-1">
			{#each data.marginChannels as channel (channel.label)}<div
					class="card border border-surface-300-700 bg-surface-50-950 p-5 shadow-sm"
				>
					<div class="flex items-center justify-between">
						<h3 class="m-0 font-bold text-surface-900-100">{channel.label}</h3>
						<Badge variant="tonal" tone={channel.tone}>{channel.target}</Badge>
					</div>
					<p class="mt-4 mb-0 text-sm text-surface-700-300">
						{channel.count} variante{channel.count > 1 ? 's' : ''} sous cible
					</p>
				</div>{/each}
		</section>
	{/if}
</PageShell>
<ProductEditModal open={editOpen} products={data.products} onClose={() => (editOpen = false)} />
<ConfirmDialog
	open={deleteTarget !== null}
	title="Supprimer ce produit ?"
	description="La suppression sera refusée si ce produit possède déjà un historique. Dans ce cas, archivez-le."
	busy={deleteBusy}
	onClose={() => (deleteTarget = null)}
	onConfirm={deleteProduct}
>
	{#if deleteTarget}<p class="m-0 text-sm text-surface-800-200">
			<strong>{deleteTarget.name}</strong> · {deleteTarget.category}
		</p>{/if}
</ConfirmDialog>
