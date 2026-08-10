import type { Tone } from '$lib/types';

export type ProductionChoice = { id: string; label: string; detail?: string };
export type PieceChoice = ProductionChoice & { capacity: number };

export type ProductionJob = {
	id: string;
	pieceId: string;
	filamentSupplyId: string;
	machineId: string | null;
	quantity: number;
	state: string;
	stateLabel: string;
	tone: Tone;
	startedAt: string | null;
	failureReason: string | null;
	failedQuantity: number | null;
	actualWasteGrams: number | null;
	pieceLabel: string;
	filamentLabel: string;
	machineLabel: string;
};

export type ProductionBundle = {
	id: string;
	reference: string;
	linkedOrderReference: string | null;
	deadline: string;
	planningPreference: string;
	state: string;
	stateLabel: string;
	tone: Tone;
	scopeLabel: string;
	completedJobs: number;
	totalJobs: number;
	jobs: ProductionJob[];
};
