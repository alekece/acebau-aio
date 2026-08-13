import { graphqlOrFallback } from '$lib/api/graphql';
import { Tone } from '$lib/components/ui/presets';
import type { PageLoad } from './$types';

type ResellerResult = {
	resellers: {
		id: string;
		status: string;
		businessName: string;
		city: string;
		country: string;
		relationship: string;
		nextActionDate: string | null;
		nextAction: string;
	}[];
};

const fallback: ResellerResult = {
	resellers: [
		{
			id: 'demo-1',
			status: 'active',
			businessName: 'Maison Dune',
			city: 'Paris',
			country: 'France',
			relationship: 'approved',
			nextActionDate: '2026-08-20',
			nextAction: 'Relancer'
		},
		{
			id: 'demo-2',
			status: 'active',
			businessName: 'Bloom Bloom',
			city: 'Lyon',
			country: 'France',
			relationship: 'approved',
			nextActionDate: null,
			nextAction: ''
		},
		{
			id: 'demo-3',
			status: 'active',
			businessName: 'Studio Sillage',
			city: 'Nantes',
			country: 'France',
			relationship: 'prospect',
			nextActionDate: '2026-08-18',
			nextAction: 'Contacter'
		}
	]
};

export const load: PageLoad = async ({ fetch, parent }) => {
	const { defaultPageSize } = await parent();
	const result = await graphqlOrFallback<ResellerResult>(
		fetch,
		`query ResellersPage($pageSize: Int!) {
		resellers(pageSize: $pageSize) { id status businessName city country relationship nextActionDate nextAction }
	}`,
		fallback,
		{ pageSize: defaultPageSize }
	);
	const label = {
		approved: 'Approuvé',
		prospect: 'Prospect',
		paused: 'En pause',
		closed: 'Clôturé',
		rejected: 'Rejeté'
	} as Record<string, string>;

	return {
		resellers: result.value.resellers.map((reseller) => ({
			id: reseller.id,
			name: reseller.businessName,
			city: `${reseller.city} · ${reseller.country}`,
			state: label[reseller.relationship] ?? reseller.relationship,
			tone:
				reseller.relationship === 'approved'
					? Tone.Success
					: reseller.relationship === 'prospect'
						? Tone.Warning
						: Tone.Surface,
			activity: 'À calculer',
			turnover: '—',
			last: '—',
			next: reseller.nextActionDate
				? `${reseller.nextAction || 'Action'} · ${reseller.nextActionDate}`
				: 'Aucune action datée'
		})),
		kpis: {
			active: result.value.resellers.filter((reseller) => reseller.relationship === 'approved')
				.length,
			turnover: '—',
			orders: '—',
			averageOrder: '—'
		},
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
