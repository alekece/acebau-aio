<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { goto } from '$app/navigation';
	import { graphql } from '$lib/api/graphql';
	import FormModal from '$lib/components/ui/FormModal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Copy from '@lucide/svelte/icons/copy';

	type ProductTemplate = {
		id: string;
		name: string;
		collection: string;
		categoryName: string;
		shortDescription: string;
		customizable: boolean;
	};

	let {
		open = false,
		products = [],
		onClose
	}: { open?: boolean; products?: ProductTemplate[]; onClose: () => void } = $props();
	let name = $state('');
	let collection = $state('');
	let category = $state('');
	let shortDescription = $state('');
	let customizable = $state(false);
	let busy = $state(false);
	let error = $state('');
	let copyOpen = $state(false);
	let selectedProductId = $state('');
	let wasOpen = false;

	$effect(() => {
		if (open && !wasOpen) {
			name = '';
			collection = '';
			category = '';
			shortDescription = '';
			customizable = false;
			copyOpen = false;
			selectedProductId = products[0]?.id ?? '';
			error = '';
		}
		wasOpen = open;
	});

	function copySelectedProduct() {
		const source = products.find((product) => product.id === selectedProductId);
		if (!source) return;
		name = source.name;
		collection = source.collection;
		category = source.categoryName;
		shortDescription = source.shortDescription;
		customizable = source.customizable;
		copyOpen = false;
	}

	async function submit() {
		busy = true;
		error = '';
		try {
			const result = await graphql<{ createProduct: { id: string } }>(
				fetch,
				`
					mutation CreateProduct($input: ProductInput!) {
						createProduct(input: $input) {
							id
						}
					}
				`,
				{ input: { name, collection, category, shortDescription, customizable } }
			);
			await invalidateAll();
			onClose();
			await goto(`/catalogue/${result.createProduct.id}`);
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Impossible de créer le produit.';
		} finally {
			busy = false;
		}
	}
</script>

<FormModal
	{open}
	title="Nouveau produit"
	description="Créez son identité. La première variante reste facultative."
	submitLabel="Créer le produit"
	{busy}
	{onClose}
	onSubmit={submit}
>
	{#if products.length > 0}
		<div class="rounded-base border border-surface-300-700 bg-surface-100-900 p-4">
			<div class="flex flex-wrap items-center justify-between gap-3">
				<div>
					<strong class="block text-sm text-surface-900-100">Partir d’un produit existant</strong>
					<small class="text-surface-700-300"
						>Copiez uniquement ses informations réutilisables.</small
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
					<Input as="select" bind:value={selectedProductId} class="min-w-0 flex-1">
						{#each products as product (product.id)}<option value={product.id}
								>{product.name}</option
							>{/each}
					</Input>
					<Button type="button" tone="tertiary" onclick={copySelectedProduct}>Utiliser</Button>
				</div>
			{/if}
		</div>
	{/if}
	<label class="label">Nom français<Input required bind:value={name} /></label>
	<div class="grid grid-cols-2 gap-3 max-[600px]:grid-cols-1">
		<label class="label">Collection<Input required bind:value={collection} /></label>
		<label class="label">Catégorie<Input required bind:value={category} /></label>
	</div>
	<label class="label">Description courte<Input bind:value={shortDescription} /></label>
	<label class="flex min-h-11 items-center gap-3 text-sm text-surface-900-100">
		<input class="checkbox" type="checkbox" bind:checked={customizable} /> Personnalisation disponible
	</label>
	{#if error}<p class="rounded-base preset-tonal-error p-3 text-sm" role="alert">{error}</p>{/if}
</FormModal>
