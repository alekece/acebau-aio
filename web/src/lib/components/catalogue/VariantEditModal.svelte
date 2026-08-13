<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import Copy from '@lucide/svelte/icons/copy';
	import WandSparkles from '@lucide/svelte/icons/wand-sparkles';
	import { graphql } from '$lib/api/graphql';
	import Button from '$lib/components/ui/Button.svelte';
	import FormModal from '$lib/components/ui/FormModal.svelte';
	import Input from '$lib/components/ui/forms/Input.svelte';

	type VariantTemplate = {
		id: string;
		displayName: string;
		sku: string;
		retailPrice: string;
		resellerPrice: string;
	};

	let {
		open = false,
		product,
		variants = [],
		onClose
	}: {
		open?: boolean;
		product: { id: string; name: string; collection: string; category: string };
		variants?: VariantTemplate[];
		onClose: () => void;
	} = $props();

	let displayName = $state('');
	let sku = $state('');
	let retailPrice = $state('0€');
	let resellerPrice = $state('0€');
	let copyOpen = $state(false);
	let selectedVariantId = $state('');
	let busy = $state(false);
	let error = $state('');
	let wasOpen = false;

	$effect(() => {
		if (open && !wasOpen) {
			displayName = '';
			sku = '';
			retailPrice = '0€';
			resellerPrice = '0€';
			copyOpen = false;
			selectedVariantId = variants[0]?.id ?? '';
			error = '';
		}
		wasOpen = open;
	});

	function code(value: string, length: number) {
		return value
			.normalize('NFD')
			.replace(/[\u0300-\u036f]/g, '')
			.replace(/[^a-zA-Z0-9]/g, '')
			.slice(0, length)
			.toUpperCase();
	}

	function generateSku() {
		sku = `${code(product.collection, 3)}-${code(product.category, 4)}`;
	}

	function copySelectedVariant() {
		const source = variants.find((variant) => variant.id === selectedVariantId);
		if (!source) return;
		displayName = source.displayName;
		sku = '';
		retailPrice = source.retailPrice;
		resellerPrice = source.resellerPrice;
		copyOpen = false;
	}

	async function submit() {
		busy = true;
		error = '';
		try {
			await graphql(
				fetch,
				`
					mutation CreateVariant($input: VariantInput!) {
						createVariant(input: $input) {
							id
						}
					}
				`,
				{
					input: {
						productId: product.id,
						displayName,
						sku,
						retailPrice,
						resellerPrice
					}
				}
			);
			await invalidateAll();
			onClose();
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Impossible de créer la variante.';
		} finally {
			busy = false;
		}
	}
</script>

<FormModal
	{open}
	title="Nouvelle variante"
	description="Créez une variante vide. Sa recette et ses attributs pourront être complétés ensuite."
	submitLabel="Créer la variante"
	{busy}
	{onClose}
	onSubmit={submit}
>
	{#if variants.length > 0}
		<div class="rounded-base border border-surface-300-700 bg-surface-100-900 p-4">
			<div class="flex flex-wrap items-center justify-between gap-3">
				<div>
					<strong class="block text-sm text-surface-900-100">Partir d’une variante existante</strong
					>
					<small class="text-surface-700-300">Le SKU et l’historique ne sont jamais copiés.</small>
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
					<Input as="select" bind:value={selectedVariantId} class="min-w-0 flex-1">
						{#each variants as variant (variant.id)}<option value={variant.id}
								>{variant.displayName} · {variant.sku}</option
							>{/each}
					</Input>
					<Button type="button" tone="tertiary" onclick={copySelectedVariant}>Utiliser</Button>
				</div>
			{/if}
		</div>
	{/if}
	<label class="label"
		>Nom affiché<Input
			required
			bind:value={displayName}
			placeholder={`${product.name} · Bleu`}
		/></label
	>
	<label class="label">
		SKU
		<div class="flex gap-2">
			<Input required bind:value={sku} class="min-w-0 flex-1" />
			<Button variant="outlined" tone="surface" type="button" onclick={generateSku}
				><WandSparkles size={16} />Générer</Button
			>
		</div>
	</label>
	<div class="grid grid-cols-2 gap-3 max-[600px]:grid-cols-1">
		<label class="label">Prix public TTC<Input required bind:value={retailPrice} /></label>
		<label class="label">Prix revendeur HT<Input required bind:value={resellerPrice} /></label>
	</div>
	{#if error}<p class="rounded-base preset-tonal-error p-3 text-sm" role="alert">{error}</p>{/if}
</FormModal>
