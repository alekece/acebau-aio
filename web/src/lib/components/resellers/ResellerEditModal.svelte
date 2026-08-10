<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { graphql } from '$lib/api/graphql';
	import FormModal from '$lib/components/ui/FormModal.svelte';
	import Input from '$lib/components/ui/Input.svelte';

	let { open = false, onClose }: { open?: boolean; onClose: () => void } = $props();
	let businessName = $state('');
	let city = $state('');
	let country = $state('France');
	let primaryEmail = $state('');
	let relationship = $state('prospect');
	let busy = $state(false);
	let error = $state('');

	async function submit() {
		busy = true;
		error = '';
		try {
			await graphql(
				fetch,
				`
					mutation CreateReseller($input: ResellerInput!) {
						createReseller(input: $input) {
							id
						}
					}
				`,
				{
					input: {
						businessName,
						city,
						country,
						primaryEmail,
						relationship,
						nextActionDate: null,
						nextAction: ''
					}
				}
			);
			await invalidateAll();
			onClose();
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Impossible de créer le revendeur.';
		} finally {
			busy = false;
		}
	}
</script>

<FormModal
	{open}
	title="Ajouter un revendeur"
	description="Créez la fiche commerciale et son contact principal."
	submitLabel="Créer le revendeur"
	{busy}
	{onClose}
	onSubmit={submit}
>
	<label class="label">Entreprise<Input required bind:value={businessName} /></label>
	<div class="grid grid-cols-2 gap-3 max-[600px]:grid-cols-1">
		<label class="label">Ville<Input required bind:value={city} /></label>
		<label class="label">Pays<Input required bind:value={country} /></label>
	</div>
	<label class="label"
		>E-mail principal<Input required type="email" bind:value={primaryEmail} /></label
	>
	<label class="label"
		>Relation<Input as="select" bind:value={relationship}
			><option value="prospect">Prospect</option><option value="approved">Approuvé</option><option
				value="paused">En pause</option
			></Input
		></label
	>
	{#if error}<p class="rounded-base preset-tonal-error p-3 text-sm" role="alert">{error}</p>{/if}
</FormModal>
