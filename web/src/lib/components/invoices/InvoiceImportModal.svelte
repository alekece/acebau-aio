<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { graphql } from '$lib/api/graphql';
	import FormModal from '$lib/components/ui/FormModal.svelte';
	import Input from '$lib/components/ui/Input.svelte';

	let { open = false, onClose }: { open?: boolean; onClose: () => void } = $props();
	let externalPlatform = $state('');
	let externalIdentifier = $state('');
	let number = $state('');
	let customer = $state('');
	let issuedOn = $state(new Date().toISOString().slice(0, 10));
	let dueOn = $state(new Date().toISOString().slice(0, 10));
	let totalHt = $state(0);
	let totalVat = $state(0);
	let busy = $state(false);
	let error = $state('');

	async function submit() {
		busy = true;
		error = '';
		try {
			await graphql(
				fetch,
				`
					mutation ImportInvoice($input: ImportedInvoiceInput!) {
						createImportedInvoice(input: $input) {
							id
						}
					}
				`,
				{
					input: {
						externalPlatform,
						externalIdentifier,
						number,
						customer,
						activityId: null,
						orderId: null,
						issuedOn,
						dueOn,
						totalHt: `${totalHt}€`,
						totalVat: `${totalVat}€`,
						totalTtc: `${totalHt + totalVat}€`,
						paymentState: 'imported'
					}
				}
			);
			await invalidateAll();
			onClose();
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Impossible d’importer la facture.';
		} finally {
			busy = false;
		}
	}
</script>

<FormModal
	{open}
	title="Importer une facture"
	description="Enregistrez les métadonnées de la facture externe faisant autorité."
	submitLabel="Importer"
	{busy}
	{onClose}
	onSubmit={submit}
>
	<div class="grid grid-cols-2 gap-3 max-[600px]:grid-cols-1">
		<label class="label">Plateforme<Input required bind:value={externalPlatform} /></label>
		<label class="label"
			>Identifiant externe<Input required bind:value={externalIdentifier} /></label
		>
	</div>
	<label class="label">Numéro<Input required bind:value={number} /></label>
	<label class="label">Client<Input required bind:value={customer} /></label>
	<div class="grid grid-cols-2 gap-3 max-[600px]:grid-cols-1">
		<label class="label">Émise le<Input required type="date" bind:value={issuedOn} /></label>
		<label class="label">Échéance<Input required type="date" bind:value={dueOn} /></label>
	</div>
	<div class="grid grid-cols-2 gap-3 max-[600px]:grid-cols-1">
		<label class="label"
			>Total HT<input
				class="input"
				required
				type="number"
				min="0"
				step="0.01"
				bind:value={totalHt}
			/></label
		>
		<label class="label"
			>TVA<input
				class="input"
				required
				type="number"
				min="0"
				step="0.01"
				bind:value={totalVat}
			/></label
		>
	</div>
	<p class="m-0 text-sm text-surface-700-300">
		Total TTC calculé : {(totalHt + totalVat).toFixed(2)} €
	</p>
	{#if error}<p class="rounded-base preset-tonal-error p-3 text-sm" role="alert">{error}</p>{/if}
</FormModal>
