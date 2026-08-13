<script lang="ts">
	import Search from '@lucide/svelte/icons/search';
	import Check from '@lucide/svelte/icons/check';
	import Pencil from '@lucide/svelte/icons/pencil';
	import Trash2 from '@lucide/svelte/icons/trash-2';
	import X from '@lucide/svelte/icons/x';
	import PackagePlus from '@lucide/svelte/icons/package-plus';
	import { invalidateAll } from '$app/navigation';
	import { graphql } from '$lib/api/graphql';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import ConfirmDialog from '$lib/components/ui/ConfirmDialog.svelte';
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import IconAction from '$lib/components/ui/IconAction.svelte';
	import Kpi from '$lib/components/ui/Kpi.svelte';
	import Input from '$lib/components/ui/forms/Input.svelte';
	import ModuleHeader from '$lib/components/ui/ModuleHeader.svelte';
	import OnboardingPanel from '$lib/components/ui/OnboardingPanel.svelte';
	import PageShell from '$lib/components/ui/PageShell.svelte';
	import Table from '$lib/components/ui/Table.svelte';
	import TableSection from '$lib/components/ui/TableSection.svelte';
	import DataSourceNotice from '$lib/components/ui/DataSourceNotice.svelte';
	import PieceEditModal from '$lib/components/pieces/PieceEditModal.svelte';
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();
	let query = $state('');
	let modalOpen = $state(false);
	let edits = $state<
		Record<string, { name: string; reference: string; busy: boolean; error: string }>
	>({});
	let deleteTarget = $state<(typeof data.pieces)[number] | null>(null);
	let deleteBusy = $state(false);
	let filtered = $derived(
		data.pieces.filter((piece) =>
			`${piece.name} ${piece.ref}`.toLowerCase().includes(query.toLowerCase())
		)
	);

	function beginEdit(piece: (typeof data.pieces)[number]) {
		edits = {
			...edits,
			[piece.id]: { name: piece.name, reference: piece.ref, busy: false, error: '' }
		};
	}

	function cancelEdit(id: string) {
		const remaining = { ...edits };
		delete remaining[id];
		edits = remaining;
	}

	async function saveEdit(id: string) {
		const edit = edits[id];
		if (!edit?.name.trim() || !edit.reference.trim()) return;
		edits = { ...edits, [id]: { ...edit, busy: true, error: '' } };
		try {
			await graphql(
				fetch,
				`
					mutation UpdatePrintedPiece($id: String!, $input: PrintedPieceInput!) {
						updatePrintedPiece(id: $id, input: $input) {
							id
						}
					}
				`,
				{ id, input: { name: edit.name.trim(), reference: edit.reference.trim() } }
			);
			cancelEdit(id);
			await invalidateAll();
		} catch (cause) {
			edits = {
				...edits,
				[id]: {
					...edit,
					busy: false,
					error: cause instanceof Error ? cause.message : 'Impossible de modifier la pièce.'
				}
			};
		}
	}

	async function deletePiece() {
		if (!deleteTarget) return;
		deleteBusy = true;
		try {
			await graphql(
				fetch,
				`
					mutation DeletePrintedPiece($id: String!) {
						deletePrintedPiece(id: $id)
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

<svelte:head><title>Pièces 3D — Acebau</title></svelte:head>
<PageShell>
	<ModuleHeader
		title="Pièces 3D"
		action="Nouvelle pièce"
		onAction={() => (modalOpen = true)}
	/><DataSourceNotice visible={data.usingFallback} message={data.loadError} />
	{#if data.pieces.length === 0}
		<OnboardingPanel
			title="Configurez votre bibliothèque de pièces"
			steps={['Créer une pièce', 'Ajouter les profils machines']}
			currentStep={0}
		>
			<EmptyState
				compact
				title="Créez votre première pièce réutilisable"
				description="Les pièces sont partagées par les recettes et les productions. Les profils machines peuvent être ajoutés après la création."
			>
				{#snippet icon()}<PackagePlus size={25} />{/snippet}
				{#snippet actions()}<Button tone="tertiary" onclick={() => (modalOpen = true)}
						>Créer une pièce</Button
					>{/snippet}
			</EmptyState>
		</OnboardingPanel>
	{:else}
		<section class="mb-6 grid max-w-3xl grid-cols-3 gap-3.5 max-[650px]:grid-cols-1">
			<Kpi label="Pièces actives" value={String(data.kpis.active)} /><Kpi
				label="Profils machines manquants"
				value={String(data.kpis.missingProfiles)}
				detail="À compléter avant production"
			/><Kpi label="Sous le taux cible" value={String(data.kpis.belowTarget)} />
		</section>
		<TableSection title="Bibliothèque">
			{#snippet toolbar()}
				<label class="relative"
					><Search
						size={16}
						class="text-surface-500-400 pointer-events-none absolute top-1/2 left-3 -translate-y-1/2"
					/><Input bind:value={query} placeholder="Rechercher une pièce" class="pl-9" /></label
				>
			{/snippet}
			<Table responsiveCards
				><thead
					><tr
						><th>Pièce</th><th>Profils machines</th><th>Temps de référence</th><th>Consommation</th
						><th>Réussite récente</th><th></th></tr
					></thead
				><tbody
					>{#each filtered as piece (piece.id)}<tr>
							{#if edits[piece.id]}
								<td data-label="Pièce">
									<div class="grid gap-2">
										<Input bind:value={edits[piece.id].name} aria-label="Nom de la pièce" />
										<Input bind:value={edits[piece.id].reference} aria-label="Référence" />
										{#if edits[piece.id].error}<small class="text-error-600-400" role="alert"
												>{edits[piece.id].error}</small
											>{/if}
									</div>
								</td>
							{:else}<td data-label="Pièce"
									><strong>{piece.name}</strong><small>{piece.ref}</small></td
								>{/if}
							<td data-label="Profils machines">{piece.profiles}</td><td
								data-label="Temps de référence">{piece.time}</td
							><td data-label="Consommation">{piece.filament}</td><td data-label="Réussite récente"
								><Badge variant="tonal" tone={piece.tone}>{piece.success}</Badge></td
							><td data-label="" class="mobile-card-actions">
								{#if edits[piece.id]}
									<IconAction
										label={`Confirmer la modification de ${piece.name}`}
										tone="success"
										disabled={edits[piece.id].busy}
										onclick={() => saveEdit(piece.id)}><Check size={18} /></IconAction
									>
									<IconAction
										label={`Annuler la modification de ${piece.name}`}
										tone="error"
										disabled={edits[piece.id].busy}
										onclick={() => cancelEdit(piece.id)}><X size={18} /></IconAction
									>
								{:else}
									<IconAction label={`Modifier ${piece.name}`} onclick={() => beginEdit(piece)}
										><Pencil size={18} /></IconAction
									>
									<IconAction
										label={`Supprimer ${piece.name}`}
										tone="error"
										onclick={() => (deleteTarget = piece)}><Trash2 size={18} /></IconAction
									>
								{/if}
							</td></tr
						>{:else}<tr
							><td data-label="" colspan="6" class="py-9 text-center text-surface-700-300"
								>Aucune pièce ne correspond à cette recherche.</td
							></tr
						>{/each}</tbody
				></Table
			>
		</TableSection>
	{/if}
</PageShell>
<PieceEditModal open={modalOpen} pieces={data.pieces} onClose={() => (modalOpen = false)} />
<ConfirmDialog
	open={deleteTarget !== null}
	title="Supprimer cette pièce ?"
	description="La suppression sera refusée si cette pièce est déjà utilisée. Dans ce cas, archivez-la."
	busy={deleteBusy}
	onClose={() => (deleteTarget = null)}
	onConfirm={deletePiece}
>
	{#if deleteTarget}<p class="m-0 text-sm text-surface-800-200">
			<strong>{deleteTarget.name}</strong> · {deleteTarget.ref}
		</p>{/if}
</ConfirmDialog>
