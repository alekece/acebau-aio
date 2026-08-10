import { graphqlOrFallback } from '$lib/api/graphql';
import type { ProductionBundle } from '$lib/production/types';
import { Tone } from '$lib/types';
import type { PageLoad } from './$types';

type ProductionResult = {
	productionTasks: {
		id: string;
		reference: string;
		sourceVariantId: string | null;
		productQuantity: number | null;
		linkedOrderReference: string | null;
		deadline: string;
		state: string;
		planningPreference: string;
	}[];
	productionTaskLines: {
		id: string;
		productionTaskId: string;
		pieceId: string;
		filamentSupplyId: string;
		quantity: number;
		machineId: string | null;
		state: string;
		startedAt: string | null;
		failureReason: string | null;
		failedQuantity: number | null;
		actualWasteGrams: number | null;
	}[];
	variants: { id: string; displayName: string; sku: string }[];
	printedPieces: { id: string; name: string; reference: string }[];
	recipeItems: {
		variantId: string;
		pieceId: string;
		filamentSupplyId: string;
		quantity: number;
	}[];
	pieceMachineProfiles: {
		pieceId: string;
		plateCapacity: number;
		preferred: boolean;
		excluded: boolean;
	}[];
	supplies: { id: string; name: string; reference: string; kind: string }[];
	machines: { id: string; surname: string; state: string }[];
};

const fallback: ProductionResult = {
	productionTasks: [
		{
			id: 'demo-1',
			reference: 'PRD-000268',
			sourceVariantId: 'variant-vase-blue',
			productQuantity: 4,
			linkedOrderReference: 'CMD-1044',
			deadline: '2026-08-11',
			state: 'in_progress',
			planningPreference: 'quality'
		}
	],
	productionTaskLines: [
		{
			id: 'line-1',
			productionTaskId: 'demo-1',
			pieceId: 'piece-vase',
			filamentSupplyId: 'filament-blue',
			quantity: 4,
			machineId: 'machine-atlas',
			state: 'printing',
			startedAt: '2026-08-11T08:30:00Z',
			failureReason: null,
			failedQuantity: null,
			actualWasteGrams: null
		},
		...Array.from({ length: 4 }, (_, index) => ({
			id: `line-insert-${index}`,
			productionTaskId: 'demo-1',
			pieceId: 'piece-insert',
			filamentSupplyId: 'filament-blue',
			quantity: 1,
			machineId: index === 0 ? 'machine-atlas' : null,
			state: 'to_print',
			startedAt: null,
			failureReason: null,
			failedQuantity: null,
			actualWasteGrams: null
		}))
	],
	variants: [{ id: 'variant-vase-blue', displayName: 'Vase ONDRA bleu', sku: 'OND-V-BL' }],
	printedPieces: [
		{ id: 'piece-vase', name: 'Corps vase ONDRA', reference: 'PIE-0018' },
		{ id: 'piece-insert', name: 'Insert vase ONDRA', reference: 'PIE-0019' }
	],
	recipeItems: [
		{
			variantId: 'variant-vase-blue',
			pieceId: 'piece-vase',
			filamentSupplyId: 'filament-blue',
			quantity: 1
		},
		{
			variantId: 'variant-vase-blue',
			pieceId: 'piece-insert',
			filamentSupplyId: 'filament-blue',
			quantity: 1
		}
	],
	pieceMachineProfiles: [
		{ pieceId: 'piece-vase', plateCapacity: 4, preferred: true, excluded: false },
		{ pieceId: 'piece-insert', plateCapacity: 1, preferred: true, excluded: false }
	],
	supplies: [{ id: 'filament-blue', name: 'PLA bleu', reference: 'FIL-BL', kind: 'filament' }],
	machines: [{ id: 'machine-atlas', surname: 'Atlas', state: 'running' }]
};

const stateLabel: Record<string, string> = {
	to_plan: 'À planifier',
	to_start: 'À lancer',
	in_progress: 'En cours',
	completed: 'Terminée',
	failed: 'Échec',
	cancelled: 'Annulée'
};

const stateTone: Record<string, Tone> = {
	to_plan: Tone.Surface,
	to_start: Tone.Warning,
	in_progress: Tone.Secondary,
	completed: Tone.Success,
	failed: Tone.Error,
	cancelled: Tone.Surface
};

const jobStateLabel: Record<string, string> = {
	to_print: 'À imprimer',
	printing: 'Impression',
	done: 'Terminée',
	failed: 'Échec'
};

const jobStateTone: Record<string, Tone> = {
	to_print: Tone.Surface,
	printing: Tone.Secondary,
	done: Tone.Success,
	failed: Tone.Error
};

export const load: PageLoad = async ({ fetch, parent }) => {
	const { defaultPageSize } = await parent();
	const result = await graphqlOrFallback<ProductionResult>(
		fetch,
		`query ProductionPage($pageSize: Int!) {
			productionTasks(pageSize: $pageSize) {
				id reference sourceVariantId productQuantity linkedOrderReference deadline state planningPreference
			}
			productionTaskLines(pageSize: $pageSize) {
				id productionTaskId pieceId filamentSupplyId quantity machineId state startedAt
				failureReason failedQuantity actualWasteGrams
			}
			variants(pageSize: $pageSize) { id displayName sku }
			printedPieces(pageSize: $pageSize) { id name reference }
			recipeItems(pageSize: $pageSize) { variantId pieceId filamentSupplyId quantity }
			pieceMachineProfiles(pageSize: $pageSize) { pieceId plateCapacity preferred excluded }
			supplies(pageSize: $pageSize) { id name reference kind }
			machines(pageSize: $pageSize) { id surname state }
		}`,
		fallback,
		{ pageSize: defaultPageSize }
	);

	const capacityFor = (pieceId: string) => {
		const profiles = result.value.pieceMachineProfiles.filter(
			(profile) => profile.pieceId === pieceId && !profile.excluded
		);
		return (
			profiles.find((profile) => profile.preferred)?.plateCapacity ??
			Math.max(1, ...profiles.map((profile) => profile.plateCapacity))
		);
	};

	const tasks: ProductionBundle[] = result.value.productionTasks.map((task) => {
		const lines = result.value.productionTaskLines.filter(
			(line) => line.productionTaskId === task.id
		);
		const sourceVariant = result.value.variants.find(
			(variant) => variant.id === task.sourceVariantId
		);
		const names = [
			...new Set(
				lines.map(
					(line) =>
						result.value.printedPieces.find((piece) => piece.id === line.pieceId)?.name ??
						'Élément archivé'
				)
			)
		];
		const jobs = lines.map((line) => {
			const piece = result.value.printedPieces.find((candidate) => candidate.id === line.pieceId);
			const filament = result.value.supplies.find(
				(candidate) => candidate.id === line.filamentSupplyId
			);
			const machine = result.value.machines.find((candidate) => candidate.id === line.machineId);
			return {
				...line,
				stateLabel: jobStateLabel[line.state] ?? line.state,
				tone: jobStateTone[line.state] ?? Tone.Surface,
				pieceLabel: piece?.name ?? 'Pièce archivée',
				filamentLabel: filament?.name ?? 'Filament archivé',
				machineLabel: machine?.surname ?? 'À affecter'
			};
		});
		const scopeLabel = task.linkedOrderReference
			? `Commande ${task.linkedOrderReference}`
			: names.length === 1
				? names[0]
				: sourceVariant
					? `${sourceVariant.displayName}${task.productQuantity ? ` × ${task.productQuantity}` : ''}`
					: names.length
						? `${names.length} pièces différentes`
						: 'Production libre';

		return {
			id: task.id,
			reference: task.reference,
			linkedOrderReference: task.linkedOrderReference,
			deadline: task.deadline,
			planningPreference: task.planningPreference,
			state: task.state,
			stateLabel: stateLabel[task.state] ?? task.state,
			tone: stateTone[task.state] ?? Tone.Surface,
			scopeLabel,
			completedJobs: jobs.filter((job) => job.state === 'done').length,
			totalJobs: jobs.length,
			jobs
		};
	});

	return {
		tasks,
		variants: result.value.variants
			.map((variant) => {
				const recipe = result.value.recipeItems.filter((item) => item.variantId === variant.id);
				return {
					id: variant.id,
					label: `${variant.displayName} · ${variant.sku}`,
					detail: `${recipe.length} type${recipe.length > 1 ? 's' : ''} de pièce`,
					recipe: recipe.map((item) => ({
						pieceId: item.pieceId,
						filamentSupplyId: item.filamentSupplyId,
						quantity: item.quantity
					}))
				};
			})
			.filter((variant) => variant.recipe.length > 0),
		pieces: result.value.printedPieces.map((piece) => ({
			id: piece.id,
			label: piece.name,
			detail: piece.reference,
			capacity: capacityFor(piece.id)
		})),
		filaments: result.value.supplies
			.filter((supply) => supply.kind === 'filament')
			.map((supply) => ({ id: supply.id, label: supply.name, detail: supply.reference })),
		machines: result.value.machines.map((machine) => ({
			id: machine.id,
			label: machine.surname,
			detail: machine.state
		})),
		machineLoad: result.value.machines.map((machine) => ({
			name: machine.surname,
			percentage: machine.state === 'running' ? '100 %' : '0 %',
			tone: machine.state === 'running' ? Tone.Error : Tone.Success
		})),
		kpis: {
			toStart: tasks.filter((task) => task.state === 'to_start').length,
			load: '—',
			threatened: '—',
			success: '—',
			waste: '—'
		},
		readiness: [
			{
				label: 'Créer une pièce imprimée',
				ready: result.value.printedPieces.length > 0,
				href: '/pieces'
			},
			{
				label: 'Ajouter un filament',
				ready: result.value.supplies.some((supply) => supply.kind === 'filament'),
				href: '/inventory'
			},
			{
				label: 'Ajouter une machine',
				ready: result.value.machines.length > 0,
				href: '/machines'
			}
		],
		incidents: [],
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
