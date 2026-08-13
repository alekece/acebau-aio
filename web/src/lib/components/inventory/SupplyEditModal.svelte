<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { graphql } from '$lib/api/graphql';
	import MetricInput from '$lib/components/forms/MetricInput.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import FormModal from '$lib/components/ui/FormModal.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Copy from '@lucide/svelte/icons/copy';

	type SupplyTemplate = {
		id: string;
		name: string;
		reference: string;
		kind: string;
		baseUnit: string;
		availableQuantity: number;
		lowStockThreshold: number;
		targetQuantity: number;
	};

	let {
		open = false,
		supplies = [],
		onClose
	}: { open?: boolean; supplies?: SupplyTemplate[]; onClose: () => void } = $props();
	let name = $state('');
	let reference = $state('');
	let kind = $state('filament');
	let baseUnit = $state('');
	let availableQuantity = $state('0');
	let lowStockThreshold = $state('0');
	let targetQuantity = $state('0');
	let busy = $state(false);
	let error = $state('');
	let copyOpen = $state(false);
	let selectedSupplyId = $state('');
	let wasOpen = false;
	const massUnits = [
		{ value: 'g', label: 'g' },
		{ value: 'kg', label: 'kg' }
	];
	const countUnits = [
		{ value: 'piece', label: 'pièce' },
		{ value: 'm', label: 'm' },
		{ value: 'ml', label: 'ml' }
	];
	let units = $derived(kind === 'filament' ? massUnits : countUnits);

	$effect(() => {
		if (open && !wasOpen) {
			name = '';
			reference = '';
			kind = 'filament';
			baseUnit = '';
			availableQuantity = '0';
			lowStockThreshold = '0';
			targetQuantity = '0';
			copyOpen = false;
			selectedSupplyId = supplies[0]?.id ?? '';
			error = '';
		}
		wasOpen = open;
	});

	$effect(() => {
		if (!units.some((unit) => unit.value === baseUnit)) baseUnit = '';
	});

	function copySelectedSupply() {
		const source = supplies.find((supply) => supply.id === selectedSupplyId);
		if (!source) return;
		name = source.name;
		reference = '';
		kind = source.kind;
		baseUnit = source.baseUnit;
		availableQuantity = '0';
		lowStockThreshold = source.lowStockThreshold.toString();
		targetQuantity = source.targetQuantity.toString();
		copyOpen = false;
	}

	async function submit() {
		busy = true;
		error = '';
		try {
			await graphql(
				fetch,
				`
					mutation CreateSupply($input: SupplyInput!) {
						createSupply(input: $input) {
							id
						}
					}
				`,
				{
					input: {
						name,
						reference,
						kind,
						baseUnit,
						availableQuantity: Number(availableQuantity),
						lowStockThreshold: Number(lowStockThreshold),
						targetQuantity: Number(targetQuantity)
					}
				}
			);
			await invalidateAll();
			onClose();
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Impossible de créer l’article.';
		} finally {
			busy = false;
		}
	}
</script>

<FormModal
	{open}
	title="Ajouter un article"
	description="Les quantités restent exprimées dans l’unité de base choisie."
	submitLabel="Créer l’article"
	{busy}
	{onClose}
	onSubmit={submit}
>
	{#if supplies.length > 0}
		<div class="rounded-base border border-surface-300-700 bg-surface-100-900 p-4">
			<div class="flex flex-wrap items-center justify-between gap-3">
				<div>
					<strong class="block text-sm text-surface-900-100">Partir d’un article existant</strong>
					<small class="text-surface-700-300"
						>Copiez son unité et ses seuils, sans copier son stock.</small
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
					<Input as="select" bind:value={selectedSupplyId} class="min-w-0 flex-1">
						{#each supplies as supply (supply.id)}<option value={supply.id}
								>{supply.name} · {supply.reference}</option
							>{/each}
					</Input>
					<Button type="button" tone="tertiary" onclick={copySelectedSupply}>Utiliser</Button>
				</div>
			{/if}
		</div>
	{/if}
	<div class="grid grid-cols-2 gap-3 max-[600px]:grid-cols-1">
		<label class="label">Nom<Input required bind:value={name} /></label>
		<label class="label">Référence<Input required bind:value={reference} /></label>
	</div>
	<label class="label"
		>Type<Input as="select" bind:value={kind}
			><option value="filament">Filament</option><option value="production_material"
				>Matériau de production</option
			><option value="product_packaging">Emballage produit</option><option
				value="shipping_packaging">Emballage expédition</option
			></Input
		></label
	>
	<MetricInput
		label="Quantité initiale"
		bind:value={availableQuantity}
		bind:unit={baseUnit}
		{units}
		kind={kind === 'filament' ? 'mass' : undefined}
	/>
	<MetricInput
		label="Alerte sous"
		bind:value={lowStockThreshold}
		bind:unit={baseUnit}
		{units}
		kind={kind === 'filament' ? 'mass' : undefined}
	/>
	<MetricInput
		label="Quantité cible"
		bind:value={targetQuantity}
		bind:unit={baseUnit}
		{units}
		kind={kind === 'filament' ? 'mass' : undefined}
		min={lowStockThreshold}
	/>
	{#if error}<p class="rounded-base preset-tonal-error p-3 text-sm" role="alert">{error}</p>{/if}
</FormModal>
