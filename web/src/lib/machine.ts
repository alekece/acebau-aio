import type { MetricDTO, PowerUnit, PriceUnit, RatioDTO, TimeUnit } from '$lib/unit';

export type MachineState = 'available' | 'running' | 'maintenance' | 'broken';

export type MachineModel = {
	id: string;
	brand: string;
	name: string;
	maintenanceCost: RatioDTO<PriceUnit, TimeUnit>;
	lifetime: MetricDTO<TimeUnit>;
	averagePower: MetricDTO<PowerUnit>;
	hasCarbonFilter: boolean;
};

export type Machine = {
	id: string;
	surname: string;
	modelId: string;
	purchaseCost: MetricDTO<PriceUnit>;
	printingTime: MetricDTO<TimeUnit>;
	usageCost: RatioDTO<PriceUnit, TimeUnit>;
	state: MachineState;
	model: MachineModel;
};

export type MachinePage = {
	items: Machine[];
	page: number;
	pageSize: number;
	totalItems: number;
	totalPages: number;
};

export type MachineMaintenanceKind =
	| 'nozzle'
	| 'axisCleaning'
	| 'axisLubrication'
	| 'generalCleaning'
	| 'carbonFilter';

export type MachineMaintenanceCriticity = 'normal' | 'due' | 'critical';

export type MachineMaintenanceStatus = {
	kind: MachineMaintenanceKind;
	criticity: MachineMaintenanceCriticity;
	dueAfter: MetricDTO<TimeUnit>;
	criticalAfter: MetricDTO<TimeUnit>;
	printingTimeSinceMaintenance: MetricDTO<TimeUnit>;
	lastPerformedAt: string | null;
};

export type MachineMaintenanceSetting = {
	id: string;
	kind: MachineMaintenanceKind;
	dueAfter: MetricDTO<TimeUnit>;
	criticalAfter: MetricDTO<TimeUnit>;
};

export type MachineMaintenanceRecord = {
	id: string;
	machineId: string;
	kind: MachineMaintenanceKind;
	performedAt: string;
	printingTime: MetricDTO<TimeUnit>;
	notes: string | null;
	machine?: { id: string; surname: string };
};

export type MachineMaintenancePage = {
	items: MachineMaintenanceRecord[];
	page: number;
	pageSize: number;
	totalItems: number;
	totalPages: number;
};

export const maintenanceKindLabels: Record<MachineMaintenanceKind, string> = {
	nozzle: 'Buse',
	axisCleaning: 'Nettoyage des axes',
	axisLubrication: 'Lubrification des axes',
	generalCleaning: 'Nettoyage général',
	carbonFilter: 'Filtre à charbon'
};
