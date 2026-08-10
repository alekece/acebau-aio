import { graphqlOrFallback } from '$lib/api/graphql';
import { Tone } from '$lib/types';
import type { PageLoad } from './$types';

type OrdersResult = {
	customerOrders: {
		id: string;
		reference: string;
		customerName: string;
		source: string;
		state: string;
		requestedOn: string;
		totalHt: string;
		progressSummary: string;
	}[];
};

const fallback: OrdersResult = {
	customerOrders: [
		{
			id: 'demo-1',
			reference: 'CMD-1044',
			customerName: 'Maison Dune',
			source: 'reseller_catalogue',
			state: 'pending',
			requestedOn: '2026-08-11',
			totalHt: '842 €',
			progressSummary: 'En attente de décision'
		},
		{
			id: 'demo-2',
			reference: 'CMD-1042',
			customerName: 'Bloom Bloom',
			source: 'reseller_catalogue',
			state: 'in_production',
			requestedOn: '2026-08-08',
			totalHt: '1 260 €',
			progressSummary: '3 / 5 pièces terminées'
		},
		{
			id: 'demo-3',
			reference: 'CMD-1038',
			customerName: 'Studio Sillage',
			source: 'direct',
			state: 'ready_to_ship',
			requestedOn: '2026-07-31',
			totalHt: '420 €',
			progressSummary: 'Colis à préparer'
		}
	]
};

const stateLabels: Record<string, string> = {
	pending: 'Demande',
	accepted: 'Acceptée',
	rejected: 'Rejetée',
	in_production: 'En production',
	ready_to_ship: 'À expédier',
	awaiting_payment: 'À encaisser',
	completed: 'Terminée',
	cancelled: 'Annulée'
};
const sourceLabels: Record<string, string> = {
	reseller_catalogue: 'Catalogue revendeur',
	direct: 'Saisie manuelle',
	shopify: 'Shopify',
	etsy: 'Etsy'
};

export const load: PageLoad = async ({ fetch, parent }) => {
	const { defaultPageSize } = await parent();
	const result = await graphqlOrFallback<OrdersResult>(
		fetch,
		`query OrdersPage($pageSize: Int!) {
		customerOrders(pageSize: $pageSize) { id reference customerName source state requestedOn totalHt progressSummary }
	}`,
		fallback,
		{ pageSize: defaultPageSize }
	);
	const tone = (state: string) =>
		state === 'pending'
			? Tone.Warning
			: state === 'in_production'
				? Tone.Secondary
				: state === 'ready_to_ship' || state === 'completed'
					? Tone.Success
					: Tone.Surface;
	const action = (state: string) =>
		state === 'pending'
			? 'Décider'
			: state === 'in_production'
				? 'Suivre'
				: state === 'ready_to_ship'
					? 'Préparer'
					: 'Ouvrir';

	return {
		orders: result.value.customerOrders.map((order) => ({
			recordId: order.id,
			id: order.reference,
			customer: order.customerName,
			source: sourceLabels[order.source] ?? order.source,
			date: order.requestedOn,
			value: `${order.totalHt} HT`,
			status: stateLabels[order.state] ?? order.state,
			tone: tone(order.state),
			progress: order.progressSummary || '—',
			action: action(order.state),
			lines: 'Contenu disponible depuis le détail de la commande.'
		})),
		lifecycle: ['pending', 'accepted', 'in_production', 'ready_to_ship', 'awaiting_payment'].map(
			(state) => ({
				label: stateLabels[state],
				count: result.value.customerOrders.filter((order) => order.state === state).length,
				tone: tone(state)
			})
		),
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
