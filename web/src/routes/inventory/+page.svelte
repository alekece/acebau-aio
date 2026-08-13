<script lang="ts">
	import Check from '@lucide/svelte/icons/check';
	import ClipboardCheck from '@lucide/svelte/icons/clipboard-check';
	import FilePlus from '@lucide/svelte/icons/file-plus';
	import PackagePlus from '@lucide/svelte/icons/package-plus';
	import Pencil from '@lucide/svelte/icons/pencil';
	import Plus from '@lucide/svelte/icons/plus';
	import RotateCcw from '@lucide/svelte/icons/rotate-ccw';
	import Search from '@lucide/svelte/icons/search';
	import Trash2 from '@lucide/svelte/icons/trash-2';
	import X from '@lucide/svelte/icons/x';
	import { invalidateAll } from '$app/navigation';
	import { graphql } from '$lib/api/graphql';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import ConfirmDialog from '$lib/components/ui/ConfirmDialog.svelte';
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import IconAction from '$lib/components/ui/IconAction.svelte';
	import Input from '$lib/components/ui/forms/Input.svelte';
	import Kpi from '$lib/components/ui/Kpi.svelte';
	import ModuleHeader from '$lib/components/ui/ModuleHeader.svelte';
	import OnboardingPanel from '$lib/components/ui/OnboardingPanel.svelte';
	import PageShell from '$lib/components/ui/PageShell.svelte';
	import Table from '$lib/components/ui/Table.svelte';
	import { Tone } from '$lib/types';
	import DataSourceNotice from '$lib/components/ui/DataSourceNotice.svelte';
	import SupplyEditModal from '$lib/components/inventory/SupplyEditModal.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	type Tab = 'filaments' | 'materials' | 'products' | 'movements' | 'counts';
	let activeTab = $state<Tab>('filaments');
	let expandedFilament = $state<string | null>(null);
	let search = $state('');
	let modalOpen = $state(false);
	let normalizedSearch = $derived(search.trim().toLowerCase());
	let filteredFilaments = $derived(
		data.filaments.filter((item) =>
			`${item.name} ${item.reference}`.toLowerCase().includes(normalizedSearch)
		)
	);
	let filteredMaterials = $derived(
		data.materials.filter((item) =>
			`${item.name} ${item.reference} ${item.category}`.toLowerCase().includes(normalizedSearch)
		)
	);
	let filteredProducts = $derived(
		data.products.filter((item) =>
			`${item.name} ${item.sku}`.toLowerCase().includes(normalizedSearch)
		)
	);
	let filteredMovements = $derived(
		data.movements.filter((item) =>
			`${item.item} ${item.reference} ${item.type}`.toLowerCase().includes(normalizedSearch)
		)
	);
	let filteredCounts = $derived(
		data.counts.filter((item) =>
			`${item.scope} ${item.state}`.toLowerCase().includes(normalizedSearch)
		)
	);
	type SupplyEdit = {
		name: string;
		reference: string;
		kind: string;
		baseUnit: string;
		availableQuantity: number;
		lowStockThreshold: number;
		targetQuantity: number;
		busy: boolean;
		error: string;
	};
	let edits = $state<Record<string, SupplyEdit>>({});
	let deleteTarget = $state<(typeof data.supplyTemplates)[number] | null>(null);
	let deleteBusy = $state(false);

	const tabs: { id: Tab; label: string }[] = [
		{ id: 'filaments', label: 'Filaments' },
		{ id: 'materials', label: 'Matériaux' },
		{ id: 'products', label: 'Produits' },
		{ id: 'movements', label: 'Mouvements de stock' },
		{ id: 'counts', label: 'Comptages' }
	];

	function toggleFilament(id: string) {
		expandedFilament = expandedFilament === id ? null : id;
	}

	function handleFilamentRow(event: MouseEvent, id: string) {
		if ((event.target as HTMLElement).closest('input, select, button, a')) return;
		toggleFilament(id);
	}

	function beginEdit(supply: (typeof data.supplyTemplates)[number]) {
		edits = {
			...edits,
			[supply.id]: { ...supply, busy: false, error: '' }
		};
	}

	function beginEditById(id: string) {
		const supply = data.supplyTemplates.find((candidate) => candidate.id === id);
		if (supply) beginEdit(supply);
	}

	function requestDeleteById(id: string) {
		deleteTarget = data.supplyTemplates.find((candidate) => candidate.id === id) ?? null;
	}

	function cancelEdit(id: string) {
		const remaining = { ...edits };
		delete remaining[id];
		edits = remaining;
	}

	async function saveEdit(id: string) {
		const edit = edits[id];
		if (!edit?.name.trim() || !edit.reference.trim() || !edit.baseUnit) return;
		edits = { ...edits, [id]: { ...edit, busy: true, error: '' } };
		try {
			await graphql(
				fetch,
				`
					mutation UpdateSupply($id: String!, $input: SupplyInput!) {
						updateSupply(id: $id, input: $input) {
							id
						}
					}
				`,
				{
					id,
					input: {
						name: edit.name.trim(),
						reference: edit.reference.trim(),
						kind: edit.kind,
						baseUnit: edit.baseUnit,
						availableQuantity: Number(edit.availableQuantity),
						lowStockThreshold: Number(edit.lowStockThreshold),
						targetQuantity: Number(edit.targetQuantity)
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
					error: cause instanceof Error ? cause.message : 'Impossible de modifier l’article.'
				}
			};
		}
	}

	async function deleteSupply() {
		if (!deleteTarget) return;
		deleteBusy = true;
		try {
			await graphql(
				fetch,
				`
					mutation DeleteSupply($id: String!) {
						deleteSupply(id: $id)
					}
				`,
				{
					id: deleteTarget.id
				}
			);
			deleteTarget = null;
			await invalidateAll();
		} finally {
			deleteBusy = false;
		}
	}
</script>

<svelte:head><title>Inventaire — Acebau</title></svelte:head>

<PageShell>
	<ModuleHeader
		title="Inventaire"
		action="Ajouter un article"
		onAction={() => (modalOpen = true)}
	/>
	<DataSourceNotice visible={data.usingFallback} message={data.loadError} />

	{#if data.supplyTemplates.length > 0}<section
			class="mb-6 grid grid-cols-5 gap-3.5 max-[1100px]:grid-cols-3 max-[700px]:grid-cols-2 max-[500px]:grid-cols-1"
			aria-label="Indicateurs d’inventaire"
		>
			<Kpi
				label="Articles sous seuil"
				value={String(data.kpis.belowThreshold)}
				detail="Fournitures persistées"
			/>
			<Kpi
				label="Productions bloquées"
				value={String(data.kpis.productionBlocked)}
				detail="Approvisionnement requis"
			/>
			<Kpi
				label="Ruptures prévues"
				value={String(data.kpis.predictedShortages)}
				detail="Dans les 14 prochains jours"
			/>
			<Kpi
				label="Valeur estimée du stock"
				value={data.kpis.estimatedValue}
				detail="Coût moyen pondéré"
			/>
			<Kpi
				label="Produits Défaut"
				value={String(data.kpis.defectiveProducts)}
				detail="Sélection manuelle"
			/>
		</section>{/if}

	{#if data.supplyTemplates.length === 0}
		<OnboardingPanel
			title="Configurez votre inventaire"
			steps={['Définir un article', 'Enregistrer le stock']}
			currentStep={0}
		>
			<EmptyState
				compact
				title="Ajoutez votre premier article"
				description="Créez une fourniture vide. Vous pourrez ensuite enregistrer ses réceptions et ses bobines physiques."
			>
				{#snippet icon()}<PackagePlus size={25} />{/snippet}
				{#snippet actions()}<Button tone="tertiary" onclick={() => (modalOpen = true)}
						><Plus size={16} />Ajouter un article</Button
					>{/snippet}
			</EmptyState>
		</OnboardingPanel>
	{:else}
		<div
			class="mb-5 flex items-center justify-between gap-4 max-[700px]:flex-col max-[700px]:items-stretch"
		>
			<div
				class="flex gap-1 overflow-x-auto rounded-base bg-surface-100-900 p-1"
				aria-label="Sections de l’inventaire"
				role="tablist"
			>
				{#each tabs as tab (tab.id)}
					<button
						type="button"
						role="tab"
						aria-selected={activeTab === tab.id}
						class="rounded-base px-3 py-2 text-sm font-medium whitespace-nowrap transition {activeTab ===
						tab.id
							? 'bg-surface-50-950 text-surface-900-100 shadow-sm'
							: 'text-surface-700-300 hover:text-surface-900-100'}"
						onclick={() => (activeTab = tab.id)}>{tab.label}</button
					>
				{/each}
			</div>
			<div class="relative w-64 shrink-0 max-[700px]:w-full">
				<Search
					class="text-surface-500-400 pointer-events-none absolute top-1/2 left-3 -translate-y-1/2"
					size={16}
				/><Input class="pl-9" bind:value={search} placeholder="Rechercher dans l’inventaire" />
			</div>
		</div>

		{#if activeTab === 'filaments'}
			<section
				class="overflow-hidden card border border-surface-300-700 bg-surface-50-950 shadow-sm"
				aria-labelledby="filaments-title"
			>
				<div class="flex items-start justify-between gap-4 border-b border-surface-300-700 p-5">
					<div>
						<h2 id="filaments-title" class="m-0 text-xl font-bold text-surface-900-100">
							Filaments
						</h2>
						<p class="mt-1 mb-0 text-sm text-surface-700-300">
							Chaque bobine physique est suivie séparément pour garantir la traçabilité.
						</p>
					</div>
				</div>
				<Table responsiveCards>
					<thead
						><tr
							><th>Filament</th><th>Bobines</th><th>Quantité disponible</th><th>Alerte sous</th><th
								>État</th
							><th><span class="sr-only">Action</span></th></tr
						></thead
					>
					<tbody>
						{#each filteredFilaments as filament (filament.id)}
							<tr
								class="cursor-pointer transition hover:bg-surface-100-900"
								onclick={(event) => handleFilamentRow(event, filament.id)}
								onkeydown={(event) => event.key === 'Enter' && toggleFilament(filament.id)}
								tabindex="0"
							>
								<td data-label="Filament"
									><div class="flex items-center gap-2">
										<span class="text-surface-500-400"
											>{expandedFilament === filament.id ? '⌄' : '›'}</span
										>
										{#if edits[filament.id]}
											<div class="grid gap-2">
												<Input bind:value={edits[filament.id].name} aria-label="Nom du filament" />
												<Input
													bind:value={edits[filament.id].reference}
													aria-label="Référence du filament"
												/>
												{#if edits[filament.id].error}<small class="text-error-600-400" role="alert"
														>{edits[filament.id].error}</small
													>{/if}
											</div>
										{:else}<div>
												<strong>{filament.name}</strong><small>{filament.reference}</small>
											</div>{/if}
									</div></td
								><td data-label="Bobines">{filament.spools}</td><td data-label="Quantité disponible"
									>{#if edits[filament.id]}<Input
											type="number"
											min="0"
											bind:value={edits[filament.id].availableQuantity}
										/>{:else}{filament.available}{/if}</td
								><td data-label="Alerte sous"
									>{#if edits[filament.id]}<Input
											type="number"
											min="0"
											bind:value={edits[filament.id].lowStockThreshold}
										/>{:else}{filament.reorder}{/if}</td
								><td data-label="État"
									><Badge variant="tonal" tone={filament.tone}>{filament.state}</Badge></td
								><td
									data-label=""
									class="mobile-card-actions"
									onclick={(event) => event.stopPropagation()}
								>
									{#if edits[filament.id]}
										<IconAction
											label={`Confirmer la modification de ${filament.name}`}
											tone="success"
											disabled={edits[filament.id].busy}
											onclick={() => saveEdit(filament.id)}><Check size={18} /></IconAction
										>
										<IconAction
											label={`Annuler la modification de ${filament.name}`}
											tone="error"
											disabled={edits[filament.id].busy}
											onclick={() => cancelEdit(filament.id)}><X size={18} /></IconAction
										>
									{:else}
										<IconAction
											label={`Modifier ${filament.name}`}
											onclick={() => beginEditById(filament.id)}><Pencil size={18} /></IconAction
										>
										<IconAction
											label={`Supprimer ${filament.name}`}
											tone="error"
											onclick={() => requestDeleteById(filament.id)}
											><Trash2 size={18} /></IconAction
										>
									{/if}
								</td>
							</tr>
							{#if expandedFilament === filament.id}
								{#each filament.details as spool (spool.ref)}
									<tr class="bg-surface-100-900"
										><td data-label="Bobine" class="pl-10"
											><strong>{spool.ref}</strong><small>{spool.color}</small></td
										><td data-label="Restant" colspan="2">{spool.remaining}</td><td
											data-label="Coût">{spool.cost}</td
										><td data-label="État"
											><Badge
												variant="tonal"
												tone={spool.state === 'Ouverte' ? Tone.Success : Tone.Surface}
												>{spool.state}</Badge
											></td
										><td data-label=""></td></tr
									>
								{/each}
							{/if}
						{/each}
					</tbody>
				</Table>
			</section>
		{:else if activeTab === 'materials'}
			<section
				class="overflow-hidden card border border-surface-300-700 bg-surface-50-950 shadow-sm"
				aria-labelledby="materials-title"
			>
				<div class="flex items-start justify-between gap-4 border-b border-surface-300-700 p-5">
					<div>
						<h2 id="materials-title" class="m-0 text-xl font-bold text-surface-900-100">
							Matériaux
						</h2>
						<p class="mt-1 mb-0 text-sm text-surface-700-300">
							Matériaux de production et emballages partagent le même stock.
						</p>
					</div>
				</div>
				<Table responsiveCards
					><thead
						><tr
							><th>Article</th><th>Catégorie</th><th>Unité</th><th>Quantité</th><th>Alerte sous</th
							><th>Fournisseur</th><th>Délai</th><th>État</th><th></th></tr
						></thead
					><tbody
						>{#each filteredMaterials as material (material.id)}<tr
								><td data-label="Article"
									>{#if edits[material.id]}<div class="grid gap-2">
											<Input
												bind:value={edits[material.id].name}
												aria-label="Nom de l’article"
											/><Input
												bind:value={edits[material.id].reference}
												aria-label="Référence"
											/>{#if edits[material.id].error}<small class="text-error-600-400" role="alert"
													>{edits[material.id].error}</small
												>{/if}
										</div>{:else}<strong>{material.name}</strong><small>{material.reference}</small
										>{/if}</td
								><td data-label="Catégorie">{material.category}</td><td data-label="Unité"
									>{material.unit}</td
								><td data-label="Quantité"
									>{#if edits[material.id]}<Input
											type="number"
											min="0"
											bind:value={edits[material.id].availableQuantity}
										/>{:else}{material.quantity}{/if}</td
								><td data-label="Alerte sous"
									>{#if edits[material.id]}<Input
											type="number"
											min="0"
											bind:value={edits[material.id].lowStockThreshold}
										/>{:else}{material.reorder}{/if}</td
								><td data-label="Fournisseur">{material.supplier}</td><td data-label="Délai"
									>{material.lead}</td
								><td data-label="État"
									><Badge variant="tonal" tone={material.tone}
										>{material.tone === Tone.Warning ? 'Sous seuil' : 'À jour'}</Badge
									></td
								><td data-label="" class="mobile-card-actions">
									{#if edits[material.id]}
										<IconAction
											label={`Confirmer la modification de ${material.name}`}
											tone="success"
											disabled={edits[material.id].busy}
											onclick={() => saveEdit(material.id)}><Check size={18} /></IconAction
										>
										<IconAction
											label={`Annuler la modification de ${material.name}`}
											tone="error"
											disabled={edits[material.id].busy}
											onclick={() => cancelEdit(material.id)}><X size={18} /></IconAction
										>
									{:else}
										<IconAction
											label={`Modifier ${material.name}`}
											onclick={() => beginEditById(material.id)}><Pencil size={18} /></IconAction
										>
										<IconAction
											label={`Supprimer ${material.name}`}
											tone="error"
											onclick={() => requestDeleteById(material.id)}
											><Trash2 size={18} /></IconAction
										>
									{/if}
								</td></tr
							>{/each}</tbody
					></Table
				>
			</section>
		{:else if activeTab === 'products'}
			<section
				class="overflow-hidden card border border-surface-300-700 bg-surface-50-950 shadow-sm"
				aria-labelledby="products-title"
			>
				<div class="flex items-start justify-between gap-4 border-b border-surface-300-700 p-5">
					<div>
						<h2 id="products-title" class="m-0 text-xl font-bold text-surface-900-100">
							Produits finis
						</h2>
						<p class="mt-1 mb-0 text-sm text-surface-700-300">
							Le stock disponible correspond au physique moins les quantités réservées.
						</p>
					</div>
					<Button variant="outlined" tone="secondary"
						><RotateCcw size={16} />Correction rapide</Button
					>
				</div>
				<Table responsiveCards
					><thead
						><tr
							><th>Produit</th><th>Condition</th><th>Physique</th><th>Réservé</th><th>Disponible</th
							><th>État</th></tr
						></thead
					><tbody
						>{#each filteredProducts as product (product.sku)}<tr
								><td data-label="Produit"
									><strong>{product.name}</strong><small>{product.sku}</small></td
								><td data-label="Condition"
									><Badge
										variant="tonal"
										tone={product.condition === 'Défaut' ? Tone.Surface : Tone.Success}
										>{product.condition}</Badge
									></td
								><td data-label="Physique">{product.physical}</td><td data-label="Réservé"
									>{product.reserved}</td
								><td data-label="Disponible"><strong>{product.available}</strong></td><td
									data-label="État"
									><Badge variant="tonal" tone={product.tone}
										>{product.available === 0
											? 'Rupture'
											: product.available <= 1
												? 'Sous seuil'
												: 'À jour'}</Badge
									></td
								></tr
							>{/each}</tbody
					></Table
				>
			</section>
		{:else if activeTab === 'movements'}
			<section
				class="overflow-hidden card border border-surface-300-700 bg-surface-50-950 shadow-sm"
				aria-labelledby="movements-title"
			>
				<div class="flex items-start justify-between gap-4 border-b border-surface-300-700 p-5">
					<div>
						<h2 id="movements-title" class="m-0 text-xl font-bold text-surface-900-100">
							Mouvements de stock
						</h2>
						<p class="mt-1 mb-0 text-sm text-surface-700-300">
							Réceptions, consommations, réservations et entrées de production.
						</p>
					</div>
					<Button variant="outlined" tone="secondary"
						><FilePlus size={16} />Nouvelle correction</Button
					>
				</div>
				<Table responsiveCards
					><thead
						><tr
							><th>Date</th><th>Article</th><th>Type</th><th>Quantité</th><th>Référence</th><th
								>État</th
							></tr
						></thead
					><tbody
						>{#each filteredMovements as movement (`${movement.date}-${movement.reference}`)}<tr
								><td data-label="Date">{movement.date}</td><td data-label="Article"
									><strong>{movement.item}</strong></td
								><td data-label="Type">{movement.type}</td><td
									data-label="Quantité"
									class="font-semibold">{movement.quantity}</td
								><td data-label="Référence">{movement.reference}</td><td data-label="État"
									><Badge variant="tonal" tone={movement.tone}
										>{movement.tone === Tone.Error
											? 'Sortie'
											: movement.tone === Tone.Success
												? 'Entrée'
												: 'Réservation'}</Badge
									></td
								></tr
							>{/each}</tbody
					></Table
				>
			</section>
		{:else}
			<section
				class="overflow-hidden card border border-surface-300-700 bg-surface-50-950 shadow-sm"
				aria-labelledby="counts-title"
			>
				<div class="flex items-start justify-between gap-4 border-b border-surface-300-700 p-5">
					<div>
						<h2 id="counts-title" class="m-0 text-xl font-bold text-surface-900-100">Comptages</h2>
						<p class="mt-1 mb-0 text-sm text-surface-700-300">
							Les écarts sont justifiés et enregistrés lors de la confirmation.
						</p>
					</div>
					<Button tone="tertiary"><ClipboardCheck size={16} />Démarrer un comptage</Button>
				</div>
				<Table responsiveCards
					><thead
						><tr><th>Date</th><th>Périmètre</th><th>Articles</th><th>Écart</th><th>État</th></tr
						></thead
					><tbody
						>{#each filteredCounts as count (`${count.date}-${count.scope}`)}<tr
								><td data-label="Date">{count.date}</td><td data-label="Périmètre"
									><strong>{count.scope}</strong></td
								><td data-label="Articles">{count.items}</td><td data-label="Écart"
									>{count.difference}</td
								><td data-label="État"
									><Badge variant="tonal" tone={count.tone}>{count.state}</Badge></td
								></tr
							>{/each}</tbody
					></Table
				>
			</section>
		{/if}
	{/if}
</PageShell>
<SupplyEditModal
	open={modalOpen}
	supplies={data.supplyTemplates}
	onClose={() => (modalOpen = false)}
/>
<ConfirmDialog
	open={deleteTarget !== null}
	title="Supprimer cet article ?"
	description="La suppression sera refusée si cet article possède déjà des mouvements ou un historique."
	busy={deleteBusy}
	onClose={() => (deleteTarget = null)}
	onConfirm={deleteSupply}
>
	{#if deleteTarget}<p class="m-0 text-sm text-surface-800-200">
			<strong>{deleteTarget.name}</strong> · {deleteTarget.reference}
		</p>{/if}
</ConfirmDialog>
