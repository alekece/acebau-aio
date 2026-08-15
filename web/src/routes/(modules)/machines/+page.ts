import { graphqlOrFallback } from '$lib/api/graphql';
import type {
	MachineMaintenanceRecord,
	MachineMaintenancePage,
	MachineMaintenanceSetting,
	MachineMaintenanceStatus
} from '$lib/machine';
import type { Machine, MachineModel, MachinePage } from '$lib/machine';
import type { PageLoad } from './$types';

type MachinesResult = {
	machineModels: { items: MachineModel[] };
	machines: MachinePage;
	modelMachines: { items: Array<{ modelId: string }> };
	machineMaintenanceSettings: { items: MachineMaintenanceSetting[] };
	machineMaintenanceHistory: MachineMaintenancePage;
};

const fallback: MachinesResult = {
	machineModels: { items: [] },
	machines: { items: [], page: 1, pageSize: 10, totalItems: 0, totalPages: 0 },
	modelMachines: { items: [] },
	machineMaintenanceSettings: { items: [] },
	machineMaintenanceHistory: { items: [], page: 1, pageSize: 10, totalItems: 0, totalPages: 0 }
};

export const load: PageLoad = async ({ fetch }) => {
	const result = await graphqlOrFallback<MachinesResult>(
		fetch,
		`query MachinesPage($page: Int!, $pageSize: Int!) {
			machineModels(pageSize: 100) {
				items {
					id brand name
					maintenanceCost { value numeratorUnit denominatorUnit }
					lifetime { value unit }
					averagePower { value unit }
					hasCarbonFilter
				}
			}
			machines(page: $page, pageSize: $pageSize) {
				items {
					id surname modelId purchaseCost { value unit } nozzleSize { value unit } printingTime { value unit }
					usageCost { value numeratorUnit denominatorUnit } state
					model {
						id brand name
						maintenanceCost { value numeratorUnit denominatorUnit }
						lifetime { value unit }
						averagePower { value unit }
						hasCarbonFilter
					}
				}
				page pageSize totalItems totalPages
			}
			modelMachines: machines(pageSize: 100) { items { modelId } }
			machineMaintenanceSettings(pageSize: 10) {
				items { id kind dueAfter { value unit } criticalAfter { value unit } }
			}
			machineMaintenanceHistory(page: 1, pageSize: 10) {
				items {
					id machineId kind performedAt printingTime { value unit } notes
					machine { id surname }
				}
				page pageSize totalItems totalPages
			}
		}`,
		fallback,
		{ page: 1, pageSize: 10 }
	);

	const modelMachineCounts = result.value.modelMachines.items.reduce<Record<string, number>>(
		(counts, machine) => ({ ...counts, [machine.modelId]: (counts[machine.modelId] ?? 0) + 1 }),
		{}
	);
	const statusResult = await graphqlOrFallback<{
		machineMaintenanceOverview: Array<{ machineId: string; statuses: MachineMaintenanceStatus[] }>;
	}>(
		fetch,
		`query MachineMaintenanceOverview($machineIds: [String!]!) {
			machineMaintenanceOverview(machineIds: $machineIds) {
				machineId
				statuses {
					kind criticity lastPerformedAt
					dueAfter { value unit }
					criticalAfter { value unit }
					printingTimeSinceMaintenance { value unit }
				}
			}
		}`,
		{ machineMaintenanceOverview: [] },
		{ machineIds: result.value.machines.items.map((machine) => machine.id) }
	);
	const maintenanceStatuses = Object.fromEntries(
		statusResult.value.machineMaintenanceOverview.map((entry) => [entry.machineId, entry.statuses])
	);

	return {
		models: result.value.machineModels.items,
		machines: result.value.machines.items,
		machinePage: result.value.machines,
		modelMachineCounts,
		maintenanceSettings: result.value.machineMaintenanceSettings.items,
		maintenanceHistory: result.value.machineMaintenanceHistory,
		maintenanceStatuses,
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
