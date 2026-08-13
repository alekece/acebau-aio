import { emptyPage, graphqlOrFallback, type Page } from '$lib/api/graphql';
import { Tone } from '$lib/components/ui/presets';
import type { PageLoad } from './$types';

type ResellerResult = {
	resellers: Page<{
		id: string;
		status: string;
		businessName: string;
		city: string;
		country: string;
		relationship: string;
		nextActionDate: string | null;
		nextAction: string;
	}>;
};

const fallback: ResellerResult = {
	resellers: emptyPage([
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
	])
};

export const load: PageLoad = async ({ fetch, parent, url }) => {
	const { defaultPageSize } = await parent();
	const page = Number(url.searchParams.get('page') ?? 1);
	const result = await graphqlOrFallback<ResellerResult>(
		fetch,
		`query ResellersPage($page: Int!, $pageSize: Int!) {
		resellers(page: $page, pageSize: $pageSize) {
			items { id status businessName city country relationship nextActionDate nextAction }
			page pageSize totalItems totalPages
		}
	}`,
		fallback,
		{ page, pageSize: defaultPageSize }
	);
	const label = {
		approved: 'Approuvé',
		prospect: 'Prospect',
		paused: 'En pause',
		closed: 'Clôturé',
		rejected: 'Rejeté'
	} as Record<string, string>;

	return {
		resellers: result.value.resellers.items.map((reseller) => ({
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
			active: result.value.resellers.items.filter(
				(reseller) => reseller.relationship === 'approved'
			).length,
			turnover: '—',
			orders: '—',
			averageOrder: '—'
		},
		pagination: result.value.resellers,
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
