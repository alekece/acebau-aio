import { graphqlOrFallback } from '$lib/api/graphql';
import { Tone } from '$lib/types';
import type { PageLoad } from './$types';

type PieceResult = {
	printedPieces: { id: string; status: string; name: string; reference: string }[];
	pieceMachineProfiles: { pieceId: string; printingTime: string; filamentMass: string }[];
	machines: { id: string }[];
};

const fallback: PieceResult = {
	printedPieces: [
		{ id: 'demo-1', status: 'active', name: 'Pied ONDRA', reference: 'PIE-0018' },
		{ id: 'demo-2', status: 'active', name: 'Abat-jour Bloom', reference: 'PIE-0014' },
		{ id: 'demo-3', status: 'active', name: 'Support mural', reference: 'PIE-0021' }
	],
	pieceMachineProfiles: [
		{ pieceId: 'demo-1', printingTime: '4.33h', filamentMass: '186g' },
		{ pieceId: 'demo-2', printingTime: '3.16h', filamentMass: '142g' },
		{ pieceId: 'demo-3', printingTime: '1.08h', filamentMass: '38g' }
	],
	machines: [{ id: 'machine-1' }, { id: 'machine-2' }, { id: 'machine-3' }]
};

export const load: PageLoad = async ({ fetch, parent }) => {
	const { defaultPageSize } = await parent();
	const result = await graphqlOrFallback<PieceResult>(
		fetch,
		`query PiecesPage($pageSize: Int!) {
		printedPieces(pageSize: $pageSize) { id status name reference }
		pieceMachineProfiles(pageSize: $pageSize) { pieceId printingTime filamentMass }
		machines(pageSize: $pageSize) { id }
	}`,
		fallback,
		{ pageSize: defaultPageSize }
	);
	const pieces = result.value.printedPieces.map((piece) => {
		const profiles = result.value.pieceMachineProfiles.filter(
			(profile) => profile.pieceId === piece.id
		);
		return {
			id: piece.id,
			name: piece.name,
			ref: piece.reference,
			profiles: `${profiles.length} / ${result.value.machines.length} machines`,
			time: profiles[0]?.printingTime ?? 'À renseigner',
			filament: profiles[0]?.filamentMass ?? 'À renseigner',
			success: '—',
			tone: profiles.length === result.value.machines.length ? Tone.Success : Tone.Warning
		};
	});

	return {
		pieces,
		kpis: {
			active: result.value.printedPieces.filter((piece) => piece.status === 'active').length,
			missingProfiles: pieces.filter(
				(piece) => !piece.profiles.startsWith(`${result.value.machines.length} /`)
			).length,
			belowTarget: 0
		},
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
