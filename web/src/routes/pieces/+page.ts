import { emptyPage, graphqlOrFallback, type Page } from '$lib/api/graphql';
import { Tone } from '$lib/components/ui/presets';
import type { PageLoad } from './$types';

type PieceResult = {
	printedPieces: Page<{ id: string; status: string; name: string; reference: string }>;
	pieceMachineProfiles: Page<{ pieceId: string; printingTime: string; filamentMass: string }>;
	machines: Page<{ id: string }>;
};

const fallback: PieceResult = {
	printedPieces: emptyPage([
		{ id: 'demo-1', status: 'active', name: 'Pied ONDRA', reference: 'PIE-0018' },
		{ id: 'demo-2', status: 'active', name: 'Abat-jour Bloom', reference: 'PIE-0014' },
		{ id: 'demo-3', status: 'active', name: 'Support mural', reference: 'PIE-0021' }
	]),
	pieceMachineProfiles: emptyPage([
		{ pieceId: 'demo-1', printingTime: '4.33h', filamentMass: '186g' },
		{ pieceId: 'demo-2', printingTime: '3.16h', filamentMass: '142g' },
		{ pieceId: 'demo-3', printingTime: '1.08h', filamentMass: '38g' }
	]),
	machines: emptyPage([{ id: 'machine-1' }, { id: 'machine-2' }, { id: 'machine-3' }])
};

export const load: PageLoad = async ({ fetch, parent, url }) => {
	const { defaultPageSize } = await parent();
	const page = Number(url.searchParams.get('page') ?? 1);
	const result = await graphqlOrFallback<PieceResult>(
		fetch,
		`query PiecesPage($page: Int!, $pageSize: Int!) {
		printedPieces(page: $page, pageSize: $pageSize) {
			items { id status name reference }
			page pageSize totalItems totalPages
		}
		pieceMachineProfiles(pageSize: 100) { items { pieceId printingTime filamentMass } }
		machines(pageSize: 100) { items { id } }
	}`,
		fallback,
		{ page, pageSize: defaultPageSize }
	);
	const pieces = result.value.printedPieces.items.map((piece) => {
		const profiles = result.value.pieceMachineProfiles.items.filter(
			(profile) => profile.pieceId === piece.id
		);
		return {
			id: piece.id,
			name: piece.name,
			ref: piece.reference,
			profiles: `${profiles.length} / ${result.value.machines.items.length} machines`,
			time: profiles[0]?.printingTime ?? 'À renseigner',
			filament: profiles[0]?.filamentMass ?? 'À renseigner',
			success: '—',
			tone: profiles.length === result.value.machines.items.length ? Tone.Success : Tone.Warning
		};
	});

	return {
		pieces,
		pagination: result.value.printedPieces,
		kpis: {
			active: result.value.printedPieces.items.filter((piece) => piece.status === 'active').length,
			missingProfiles: pieces.filter(
				(piece) => !piece.profiles.startsWith(`${result.value.machines.items.length} /`)
			).length,
			belowTarget: 0
		},
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
