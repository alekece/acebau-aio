import { emptyPage, graphqlOrFallback, type Page } from '$lib/api/graphql';
import type { LayoutLoad } from './$types';

type CatalogueDetailResult = {
	product: {
		id: string;
		status: string;
		name: string;
		collection: string;
		category: string;
		shortDescription: string;
		customizable: boolean;
	};
	variants: Page<{
		id: string;
		productId: string;
		status: string;
		displayName: string;
		sku: string;
		retailPrice: string;
		resellerPrice: string;
	}>;
};

export const load: LayoutLoad = async ({ fetch, params, parent }) => {
	const { defaultPageSize } = await parent();
	const fallback: CatalogueDetailResult = {
		product: {
			id: params.id,
			status: 'active',
			name: params.id === 'demo-ondra' ? 'ONDRA' : 'Produit de démonstration',
			collection: 'Collection 2026',
			category: 'Éclairage',
			shortDescription: 'La fiche complète sera synchronisée lorsque l’API sera disponible.',
			customizable: true
		},
		variants: emptyPage([
			{
				id: 'demo-variant',
				productId: params.id,
				status: 'active',
				displayName: 'ONDRA · S',
				sku: 'OND-S-BL',
				retailPrice: '89 €',
				resellerPrice: '42 €'
			}
		])
	};
	const result = await graphqlOrFallback<CatalogueDetailResult>(
		fetch,
		`query CatalogueDetail($id: String!, $pageSize: Int!) {
			product(id: $id) { id status name collection category shortDescription customizable }
			variants(pageSize: $pageSize) { items { id productId status displayName sku retailPrice resellerPrice } }
		}`,
		fallback,
		{ id: params.id, pageSize: defaultPageSize }
	);

	return {
		product: result.value.product,
		variants: result.value.variants.items.filter((variant) => variant.productId === params.id),
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
