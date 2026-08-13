<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { graphql } from '$lib/api/graphql';
	import FormModal from '$lib/components/ui/FormModal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Input from '$lib/components/ui/forms/Input.svelte';
	import Copy from '@lucide/svelte/icons/copy';
	import WandSparkles from '@lucide/svelte/icons/wand-sparkles';

	let {
		open = false,
		pieces = [],
		onClose
	}: {
		open?: boolean;
		pieces?: { id: string; name: string; ref: string }[];
		onClose: () => void;
	} = $props();
	let name = $state('');
	let reference = $state('');
	let copyOpen = $state(false);
	let selectedPieceId = $state('');
	let busy = $state(false);
	let error = $state('');
	let wasOpen = false;

	$effect(() => {
		if (open && !wasOpen) {
			name = '';
			reference = '';
			copyOpen = false;
			selectedPieceId = pieces[0]?.id ?? '';
			error = '';
		}
		wasOpen = open;
	});

	function copySelectedPiece() {
		const source = pieces.find((piece) => piece.id === selectedPieceId);
		if (!source) return;
		name = source.name;
		reference = '';
		copyOpen = false;
	}

	function generateReference() {
		const next =
			Math.max(0, ...pieces.map((piece) => Number(piece.ref.match(/(\d+)$/)?.[1] ?? 0))) + 1;
		reference = `PIE-${String(next).padStart(4, '0')}`;
	}

	async function submit() {
		busy = true;
		error = '';
		try {
			await graphql(
				fetch,
				`
					mutation CreatePrintedPiece($input: PrintedPieceInput!) {
						createPrintedPiece(input: $input) {
							id
						}
					}
				`,
				{ input: { name, reference } }
			);
			await invalidateAll();
			onClose();
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Impossible de créer la pièce.';
		} finally {
			busy = false;
		}
	}
</script>

<FormModal
	{open}
	title="Nouvelle pièce"
	description="Créez une définition vide, puis ajoutez ses profils machines."
	submitLabel="Créer la pièce"
	{busy}
	{onClose}
	onSubmit={submit}
>
	{#if pieces.length > 0}
		<div class="rounded-base border border-surface-300-700 bg-surface-100-900 p-4">
			<div class="flex flex-wrap items-center justify-between gap-3">
				<div>
					<strong class="block text-sm text-surface-900-100">Partir d’une pièce existante</strong>
					<small class="text-surface-700-300"
						>Copiez ses champs réutilisables dans ce formulaire vide.</small
					>
				</div>
				<Button
					variant="outlined"
					tone="surface"
					type="button"
					onclick={() => (copyOpen = !copyOpen)}><Copy size={16} />Copier</Button
				>
			</div>
			{#if copyOpen}
				<div class="mt-4 flex gap-2 max-[520px]:flex-col">
					<Input as="select" bind:value={selectedPieceId} class="min-w-0 flex-1">
						{#each pieces as piece (piece.id)}<option value={piece.id}
								>{piece.name} · {piece.ref}</option
							>{/each}
					</Input>
					<Button type="button" tone="tertiary" onclick={copySelectedPiece}>Utiliser</Button>
				</div>
			{/if}
		</div>
	{/if}
	<label class="label">Nom<Input required bind:value={name} /></label>
	<label class="label">
		Référence
		<div class="flex gap-2">
			<Input required bind:value={reference} placeholder="PIE-0025" class="min-w-0 flex-1" />
			<Button variant="outlined" tone="surface" type="button" onclick={generateReference}
				><WandSparkles size={16} />Générer</Button
			>
		</div>
	</label>
	{#if error}<p class="rounded-base preset-tonal-error p-3 text-sm" role="alert">{error}</p>{/if}
</FormModal>
