import { emptyPage, graphqlOrFallback, type Page } from '$lib/api/graphql';
import { Tone } from '$lib/components/ui/presets';
import type { PageLoad } from './$types';

type InventoryResult = {
	supplies: Page<{
		id: string;
		name: string;
		reference: string;
		kind: string;
		baseUnit: string;
		availableQuantity: number;
		lowStockThreshold: number;
		targetQuantity: number;
	}>;
	filamentSpools: Page<{
		supplyId: string;
		internalReference: string;
		remainingWeight: string;
		receivedCost: string;
		state: string;
	}>;
};

const fallback: InventoryResult = {
	supplies: emptyPage([
		{
			id: 'demo-1',
			name: 'PLA blanc mat',
			reference: 'FIL-PLA-WHT',
			kind: 'filament',
			baseUnit: 'g',
			availableQuantity: 3200,
			lowStockThreshold: 1000,
			targetQuantity: 5000
		},
		{
			id: 'demo-2',
			name: 'PLA sable',
			reference: 'FIL-PLA-SND',
			kind: 'filament',
			baseUnit: 'g',
			availableQuantity: 600,
			lowStockThreshold: 1000,
			targetQuantity: 4000
		},
		{
			id: 'demo-3',
			name: 'Carton d’expédition S',
			reference: 'EMB-CAR-S',
			kind: 'shipping_packaging',
			baseUnit: 'piece',
			availableQuantity: 18,
			lowStockThreshold: 20,
			targetQuantity: 60
		}
	]),
	filamentSpools: emptyPage([
		{
			supplyId: 'demo-1',
			internalReference: 'BOB-024',
			remainingWeight: '1200g',
			receivedCost: '25€',
			state: 'open'
		},
		{
			supplyId: 'demo-1',
			internalReference: 'BOB-031',
			remainingWeight: '2000g',
			receivedCost: '42€',
			state: 'sealed'
		},
		{
			supplyId: 'demo-2',
			internalReference: 'BOB-018',
			remainingWeight: '600g',
			receivedCost: '22€',
			state: 'open'
		}
	])
};

export const load: PageLoad = async ({ fetch, parent, url }) => {
	const { defaultPageSize } = await parent();
	const page = Number(url.searchParams.get('page') ?? 1);
	const result = await graphqlOrFallback<InventoryResult>(
		fetch,
		`query InventoryPage($page: Int!, $pageSize: Int!) {
		supplies(page: $page, pageSize: $pageSize) {
			items { id name reference kind baseUnit availableQuantity lowStockThreshold targetQuantity }
			page pageSize totalItems totalPages
		}
		filamentSpools(pageSize: 100) { items { supplyId internalReference remainingWeight receivedCost state } }
	}`,
		fallback,
		{ page, pageSize: defaultPageSize }
	);
	const category: Record<string, string> = {
		production_material: 'Matériau de production',
		product_packaging: 'Emballage produit',
		shipping_packaging: 'Emballage expédition'
	};
	const filaments = result.value.supplies.items
		.filter((supply) => supply.kind === 'filament')
		.map((supply) => ({
			id: supply.id,
			name: supply.name,
			reference: supply.reference,
			kind: supply.kind,
			baseUnit: supply.baseUnit,
			availableQuantity: supply.availableQuantity,
			lowStockThreshold: supply.lowStockThreshold,
			targetQuantity: supply.targetQuantity,
			spools: result.value.filamentSpools.items.filter((spool) => spool.supplyId === supply.id)
				.length,
			available: `${supply.availableQuantity} ${supply.baseUnit}`,
			reorder: `${supply.lowStockThreshold} ${supply.baseUnit}`,
			state: supply.availableQuantity <= supply.lowStockThreshold ? 'Alerte sous' : 'À jour',
			tone: supply.availableQuantity <= supply.lowStockThreshold ? Tone.Warning : Tone.Success,
			details: result.value.filamentSpools.items
				.filter((spool) => spool.supplyId === supply.id)
				.map((spool) => ({
					ref: spool.internalReference,
					color: supply.name,
					remaining: spool.remainingWeight,
					cost: spool.receivedCost,
					state:
						spool.state === 'open' ? 'Ouverte' : spool.state === 'sealed' ? 'Scellée' : spool.state
				}))
		}));
	const materials = result.value.supplies.items
		.filter((supply) => supply.kind !== 'filament')
		.map((supply) => ({
			id: supply.id,
			name: supply.name,
			reference: supply.reference,
			kind: supply.kind,
			category: category[supply.kind] ?? supply.kind,
			unit: supply.baseUnit,
			baseUnit: supply.baseUnit,
			availableQuantity: supply.availableQuantity,
			lowStockThreshold: supply.lowStockThreshold,
			targetQuantity: supply.targetQuantity,
			quantity: String(supply.availableQuantity),
			reorder: String(supply.lowStockThreshold),
			supplier: 'À renseigner',
			lead: '—',
			tone: supply.availableQuantity <= supply.lowStockThreshold ? Tone.Warning : Tone.Success
		}));

	return {
		filaments,
		materials,
		kpis: {
			belowThreshold: result.value.supplies.items.filter(
				(supply) => supply.availableQuantity <= supply.lowStockThreshold
			).length,
			productionBlocked: 0,
			predictedShortages: 0,
			estimatedValue: '—',
			defectiveProducts: 0
		},
		supplyTemplates: result.value.supplies.items,
		pagination: result.value.supplies,
		// Finished-stock, movement and count modules are not persisted yet; temporary page data stays in the loader.
		products: [
			{
				name: 'Lampe Ondra · Sable',
				sku: 'OND-SAB',
				condition: 'Standard',
				physical: 12,
				reserved: 4,
				available: 8,
				tone: Tone.Success
			},
			{
				name: 'Vase Noma · Défaut émail',
				sku: 'NOM-DEF',
				condition: 'Défaut',
				physical: 5,
				reserved: 0,
				available: 5,
				tone: Tone.Surface
			}
		],
		movements: [
			{
				date: 'Aujourd’hui',
				item: 'Aucun mouvement persisté',
				type: 'Information',
				quantity: '—',
				reference: '—',
				tone: Tone.Surface
			}
		],
		counts: [
			{
				date: 'À planifier',
				scope: 'Atelier complet',
				items: 0,
				difference: '—',
				state: 'Non démarré',
				tone: Tone.Surface
			}
		],
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
