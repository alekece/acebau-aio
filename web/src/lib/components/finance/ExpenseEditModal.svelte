<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { graphql } from '$lib/api/graphql';
	import FormModal from '$lib/components/ui/FormModal.svelte';
	import Input from '$lib/components/ui/forms/Input.svelte';

	let { open = false, onClose }: { open?: boolean; onClose: () => void } = $props();
	let supplier = $state('');
	let accountingDate = $state(new Date().toISOString().slice(0, 10));
	let category = $state('Fournitures');
	let description = $state('');
	let totalHt = $state(0);
	let totalVat = $state(0);
	let paymentMethod = $state('Carte bancaire');
	let busy = $state(false);
	let error = $state('');

	async function submit() {
		busy = true;
		error = '';
		try {
			await graphql(
				fetch,
				`
					mutation CreateExpense($input: ExpenseInput!) {
						createExpense(input: $input) {
							id
						}
					}
				`,
				{
					input: {
						activityId: null,
						supplier,
						accountingDate,
						category,
						description,
						totalHt: `${totalHt}€`,
						totalVat: `${totalVat}€`,
						totalTtc: `${totalHt + totalVat}€`,
						paymentState: 'paid',
						paymentMethod
					}
				}
			);
			await invalidateAll();
			onClose();
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Impossible d’enregistrer la dépense.';
		} finally {
			busy = false;
		}
	}
</script>

<FormModal
	{open}
	title="Ajouter une dépense"
	description="La TVA suggérée doit rester confirmée par l’administrateur."
	submitLabel="Enregistrer la dépense"
	{busy}
	{onClose}
	onSubmit={submit}
>
	<label class="label">Fournisseur<Input required bind:value={supplier} /></label>
	<div class="grid grid-cols-2 gap-3 max-[600px]:grid-cols-1">
		<label class="label"
			>Date comptable<Input required type="date" bind:value={accountingDate} /></label
		>
		<label class="label">Catégorie<Input required bind:value={category} /></label>
	</div>
	<label class="label">Description<Input required bind:value={description} /></label>
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
			>TVA confirmée<input
				class="input"
				required
				type="number"
				min="0"
				step="0.01"
				bind:value={totalVat}
			/></label
		>
	</div>
	<label class="label">Moyen de paiement<Input required bind:value={paymentMethod} /></label>
	{#if error}<p class="rounded-base preset-tonal-error p-3 text-sm" role="alert">{error}</p>{/if}
</FormModal>
