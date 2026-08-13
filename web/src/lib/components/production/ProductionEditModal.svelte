<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import Plus from '@lucide/svelte/icons/plus';
	import Sparkles from '@lucide/svelte/icons/sparkles';
	import Trash2 from '@lucide/svelte/icons/trash-2';
	import { graphql } from '$lib/api/graphql';
	import Button from '$lib/components/ui/Button.svelte';
	import FormModal from '$lib/components/ui/FormModal.svelte';
	import Input from '$lib/components/ui/forms/Input.svelte';
	import { expandRecipeIntoRuns } from '$lib/production/planning';

	type PieceChoice = { id: string; label: string; detail?: string; capacity: number };
	type FilamentChoice = { id: string; label: string; detail?: string };
	type VariantChoice = {
		id: string;
		label: string;
		detail?: string;
		recipe: { pieceId: string; filamentSupplyId: string; quantity: number }[];
	};
	type ProductionLine = {
		key: number;
		pieceId: string;
		filamentSupplyId: string;
		quantity: number;
	};

	let {
		open = false,
		variants = [],
		pieces = [],
		filaments = [],
		onClose
	}: {
		open?: boolean;
		variants?: VariantChoice[];
		pieces?: PieceChoice[];
		filaments?: FilamentChoice[];
		onClose: () => void;
	} = $props();
	let nextKey = 1;
	let lines = $state<ProductionLine[]>([]);
	let selectedVariantId = $state('');
	let sourceVariantId = $state('');
	let variantQuantity = $state(1);
	let linkedOrderReference = $state('');
	let deadline = $state(new Date().toISOString().slice(0, 10));
	let planningPreference = $state('quality');
	let busy = $state(false);
	let error = $state('');

	function newLine(
		pieceId = pieces[0]?.id ?? '',
		filamentSupplyId = filaments[0]?.id ?? '',
		quantity = 1
	): ProductionLine {
		return { key: nextKey++, pieceId, filamentSupplyId, quantity };
	}

	function addPieceLine() {
		lines.push(newLine());
	}

	function removeLine(key: number) {
		lines = lines.filter((line) => line.key !== key);
	}

	function populateVariant() {
		error = '';
		const variant = variants.find((candidate) => candidate.id === selectedVariantId);
		if (!variant || !variant.recipe.length) {
			error = 'Cette variante ne contient aucune pièce imprimée dans sa recette.';
			return;
		}

		for (const run of expandRecipeIntoRuns(variant.recipe, pieces, variantQuantity)) {
			lines.push(newLine(run.pieceId, run.filamentSupplyId, run.quantity));
		}
		sourceVariantId = variant.id;
	}

	$effect(() => {
		if (!variants.some((variant) => variant.id === selectedVariantId)) {
			selectedVariantId = variants[0]?.id ?? '';
		}
		for (const line of lines) {
			if (!pieces.some((piece) => piece.id === line.pieceId)) {
				line.pieceId = pieces[0]?.id ?? '';
			}
			if (!filaments.some((filament) => filament.id === line.filamentSupplyId)) {
				line.filamentSupplyId = filaments[0]?.id ?? '';
			}
		}
	});

	async function submit() {
		if (!lines.length) {
			error = 'Ajoutez au moins un passage d’impression.';
			return;
		}
		if (lines.some((line) => !line.pieceId || !line.filamentSupplyId)) {
			error = 'Sélectionnez une pièce et un filament sur chaque ligne.';
			return;
		}

		busy = true;
		error = '';
		try {
			await graphql(
				fetch,
				`
					mutation CreateProduction($input: CreateProductionInput!) {
						createProduction(input: $input) {
							id
							reference
						}
					}
				`,
				{
					input: {
						sourceVariantId: sourceVariantId || null,
						productQuantity: sourceVariantId ? Number(variantQuantity) : null,
						linkedOrderReference: linkedOrderReference || null,
						deadline,
						planningPreference,
						lines: lines.map((line) => ({
							pieceId: line.pieceId,
							filamentSupplyId: line.filamentSupplyId,
							quantity: line.quantity
						}))
					}
				}
			);
			lines = [];
			sourceVariantId = '';
			linkedOrderReference = '';
			await invalidateAll();
			onClose();
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Impossible de créer la production.';
		} finally {
			busy = false;
		}
	}
</script>

<FormModal
	{open}
	size="wide"
	title="Nouvelle production"
	description="Chaque ligne correspond à un passage d’impression. Utilisez une variante pour préparer automatiquement ses passages, ou ajoutez les pièces librement."
	submitLabel="Créer la production"
	{busy}
	{onClose}
	onSubmit={submit}
>
	<div class="rounded-base preset-tonal-surface p-3 text-sm">
		La référence sera générée automatiquement lors de la création.
	</div>
	<div class="grid grid-cols-2 gap-3 max-[600px]:grid-cols-1">
		<label class="label">Échéance<Input required type="date" bind:value={deadline} /></label>
		<label class="label"
			>Préférence<Input as="select" bind:value={planningPreference}
				><option value="quality">Qualité</option><option value="cost">Coût</option><option
					value="speed">Rapidité</option
				></Input
			></label
		>
	</div>
	<label class="label">Commande liée (facultatif)<Input bind:value={linkedOrderReference} /></label>

	<section
		class="grid gap-3 rounded-container border border-tertiary-500/30 bg-tertiary-50-950 p-4"
	>
		<div>
			<h3 class="m-0 text-sm font-bold text-surface-900-100">Préparer depuis une variante</h3>
			<p class="mt-1 mb-0 text-xs text-surface-700-300">
				La recette est découpée en passages selon la capacité plateau de chaque pièce.
			</p>
		</div>
		<div class="grid grid-cols-[minmax(0,1fr)_6rem_auto] items-end gap-2 max-[650px]:grid-cols-1">
			<label class="label"
				>Variante<Input as="select" bind:value={selectedVariantId}>
					{#each variants as variant (variant.id)}<option value={variant.id}
							>{variant.label}{variant.detail ? ` · ${variant.detail}` : ''}</option
						>{/each}
				</Input></label
			>
			<label class="label"
				>Produits<input
					class="input"
					required
					type="number"
					min="1"
					step="1"
					bind:value={variantQuantity}
				/></label
			>
			<Button variant="tonal" tone="tertiary" type="button" onclick={populateVariant}>
				<Sparkles size={16} />Remplir la table
			</Button>
		</div>
	</section>

	<div class="overflow-hidden rounded-container border border-surface-300-700">
		<div
			class="grid grid-cols-[minmax(0,1fr)_minmax(0,1fr)_6rem_2.5rem] gap-2 bg-surface-100-900 px-3 py-2 text-xs font-semibold tracking-wide text-surface-700-300 uppercase max-[650px]:hidden"
		>
			<span>Pièce</span><span>Filament / couleur</span><span>Sur plateau</span><span></span>
		</div>
		{#each lines as line, index (line.key)}
			<div
				class="grid grid-cols-[minmax(0,1fr)_minmax(0,1fr)_6rem_2.5rem] items-end gap-2 border-t border-surface-300-700 p-3 first:border-t-0 max-[650px]:grid-cols-[1fr_5rem_2.5rem]"
			>
				<label class="label max-[650px]:col-span-3"
					><span class="hidden max-[650px]:inline">Passage {index + 1} · Pièce</span><Input
						as="select"
						required
						bind:value={line.pieceId}
						>{#each pieces as piece (piece.id)}<option value={piece.id}
								>{piece.label}{piece.detail ? ` · ${piece.detail}` : ''}</option
							>{/each}</Input
					></label
				>
				<label class="label"
					><span class="hidden max-[650px]:inline">Filament</span><Input
						as="select"
						required
						bind:value={line.filamentSupplyId}
						>{#each filaments as filament (filament.id)}<option value={filament.id}
								>{filament.label}</option
							>{/each}</Input
					></label
				>
				<label class="label"
					><span class="hidden max-[650px]:inline">Sur plateau</span><input
						class="input"
						required
						type="number"
						min="1"
						step="1"
						bind:value={line.quantity}
					/></label
				>
				<button
					class="btn-icon preset-tonal-error"
					type="button"
					aria-label={`Supprimer le passage ${index + 1}`}
					onclick={() => removeLine(line.key)}><Trash2 size={16} /></button
				>
			</div>
		{:else}
			<p class="m-0 p-5 text-center text-sm text-surface-700-300">Aucun passage préparé.</p>
		{/each}
	</div>
	<Button variant="outlined" tone="secondary" type="button" onclick={addPieceLine}>
		<Plus size={16} />Ajouter une pièce directement
	</Button>
	{#if !pieces.length || !filaments.length}
		<p class="rounded-base preset-tonal-warning p-3 text-sm">
			Une pièce et un filament sont nécessaires pour planifier une production.
		</p>
	{/if}
	{#if error}<p class="rounded-base preset-tonal-error p-3 text-sm" role="alert">{error}</p>{/if}
</FormModal>
