<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { graphql } from '$lib/api/graphql';
	import FormModal from '$lib/components/ui/FormModal.svelte';
	import Input from '$lib/components/ui/Input.svelte';

	let { open = false, onClose }: { open?: boolean; onClose: () => void } = $props();
	let reference = $state('');
	let customerName = $state('');
	let source = $state('direct');
	let requestedOn = $state(new Date().toISOString().slice(0, 10));
	let totalHt = $state(0);
	let busy = $state(false);
	let error = $state('');

	async function submit() {
		busy = true;
		error = '';
		try {
			await graphql(
				fetch,
				`
					mutation CreateCustomerOrder($input: CustomerOrderInput!) {
						createCustomerOrder(input: $input) {
							id
						}
					}
				`,
				{
					input: {
						reference,
						resellerId: null,
						customerName,
						source,
						state: 'pending',
						requestedOn,
						totalHt: `${totalHt}€`,
						progressSummary: 'En attente de décision'
					}
				}
			);
			await invalidateAll();
			onClose();
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Impossible de créer la commande.';
		} finally {
			busy = false;
		}
	}
</script>

<FormModal
	{open}
	title="Saisir une commande"
	description="Créez une commande manuelle sans déclencher automatiquement la production."
	submitLabel="Créer la commande"
	{busy}
	{onClose}
	onSubmit={submit}
>
	<div class="grid grid-cols-2 gap-3 max-[600px]:grid-cols-1">
		<label class="label"
			>Référence<Input required bind:value={reference} placeholder="CMD-1045" /></label
		>
		<label class="label">Date<Input required type="date" bind:value={requestedOn} /></label>
	</div>
	<label class="label">Client<Input required bind:value={customerName} /></label>
	<label class="label"
		>Source<Input as="select" bind:value={source}
			><option value="direct">Direct</option><option value="shopify">Shopify</option><option
				value="etsy">Etsy</option
			></Input
		></label
	>
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
	{#if error}<p class="rounded-base preset-tonal-error p-3 text-sm" role="alert">{error}</p>{/if}
</FormModal>
