import { graphqlOrFallback } from '$lib/api/graphql';
import type { PageLoad } from './$types';

type OrderDetailResult = {
	customerOrder: {
		id: string;
		reference: string;
		customerName: string;
		source: string;
		state: string;
		requestedOn: string;
		totalHt: string;
		progressSummary: string;
	};
	orderLines: {
		id: string;
		orderId: string;
		variantId: string;
		quantity: number;
		unitPriceHt: string;
	}[];
	variants: { id: string; displayName: string; sku: string }[];
};

export const load: PageLoad = async ({ fetch, params, parent }) => {
	const { defaultPageSize } = await parent();
	const fallback: OrderDetailResult = {
		customerOrder: {
			id: params.id,
			reference: 'CMD-1044',
			customerName: 'Maison Dune',
			source: 'reseller_catalogue',
			state: 'pending',
			requestedOn: '2026-08-11',
			totalHt: '842 €',
			progressSummary: 'En attente de décision'
		},
		orderLines: [],
		variants: []
	};
	const result = await graphqlOrFallback<OrderDetailResult>(
		fetch,
		`query OrderDetail($id: String!, $pageSize: Int!) {
			customerOrder(id: $id) { id reference customerName source state requestedOn totalHt progressSummary }
			orderLines(pageSize: $pageSize) { id orderId variantId quantity unitPriceHt }
			variants(pageSize: $pageSize) { id displayName sku }
		}`,
		fallback,
		{ id: params.id, pageSize: defaultPageSize }
	);

	return {
		order: result.value.customerOrder,
		lines: result.value.orderLines
			.filter((line) => line.orderId === params.id)
			.map((line) => ({
				...line,
				variant: result.value.variants.find((variant) => variant.id === line.variantId)
			})),
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
