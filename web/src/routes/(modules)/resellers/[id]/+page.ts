import { emptyPage, graphqlOrFallback, type Page } from '$lib/api/graphql';
import type { PageLoad } from './$types';

type ResellerDetailResult = {
	reseller: {
		id: string;
		businessName: string;
		city: string;
		country: string;
		relationship: string;
		primaryEmail: string;
		nextActionDate: string | null;
		nextAction: string;
	};
	customerOrders: Page<{
		id: string;
		resellerId: string | null;
		reference: string;
		requestedOn: string;
		totalHt: string;
		state: string;
	}>;
};

export const load: PageLoad = async ({ fetch, params, parent }) => {
	const { defaultPageSize } = await parent();
	const fallback: ResellerDetailResult = {
		reseller: {
			id: params.id,
			businessName: 'Maison Dune',
			city: 'Paris',
			country: 'France',
			relationship: 'approved',
			primaryEmail: 'contact@maisondune.example',
			nextActionDate: '2026-08-20',
			nextAction: 'Relancer'
		},
		customerOrders: emptyPage()
	};
	const result = await graphqlOrFallback<ResellerDetailResult>(
		fetch,
		`query ResellerDetail($id: String!, $pageSize: Int!) {
			reseller(id: $id) { id businessName city country relationship primaryEmail nextActionDate nextAction }
			customerOrders(pageSize: $pageSize) { items { id resellerId reference requestedOn totalHt state } }
		}`,
		fallback,
		{ id: params.id, pageSize: defaultPageSize }
	);

	return {
		reseller: result.value.reseller,
		orders: result.value.customerOrders.items.filter((order) => order.resellerId === params.id),
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
