import { graphqlOrFallback } from '$lib/api/graphql';
import type { Machine, MachineMaintenancePage, MachineMaintenanceStatus } from '$lib/machine';
import type { PageLoad } from './$types';

type MachineDetail = Machine;

export const load: PageLoad = async ({ fetch, params, url }) => {
	const page = Number(url.searchParams.get('maintenancePage') ?? '1');
	const fallback: {
		machine: MachineDetail;
		machineMaintenanceStatuses: MachineMaintenanceStatus[];
		machineMaintenanceHistoryFor: MachineMaintenancePage;
	} = {
		machine: {
			id: params.id,
			surname: 'Machine indisponible',
			modelId: '',
			purchaseCost: { value: '0', unit: '€' },
			nozzleSize: { value: '0.4', unit: 'mm' },
			printingTime: { value: '0', unit: 'h' },
			usageCost: { value: '0', numeratorUnit: '€', denominatorUnit: 'h' },
			state: 'available',
			model: {
				id: '',
				brand: '—',
				name: 'Modèle indisponible',
				maintenanceCost: { value: '0', numeratorUnit: '€', denominatorUnit: 'h' },
				lifetime: { value: '0', unit: 'h' },
				averagePower: { value: '0', unit: 'W' },
				hasCarbonFilter: false
			}
		},
		machineMaintenanceStatuses: [],
		machineMaintenanceHistoryFor: { items: [], page: 1, pageSize: 10, totalItems: 0, totalPages: 0 }
	};
	const result = await graphqlOrFallback<typeof fallback>(
		fetch,
		`query MachineDetail($id: String!, $page: Int!) {
			machine(id: $id) {
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
			machineMaintenanceStatuses(machineId: $id) {
				kind criticity lastPerformedAt
				dueAfter { value unit }
				criticalAfter { value unit }
				printingTimeSinceMaintenance { value unit }
			}
			machineMaintenanceHistoryFor(machineId: $id, page: $page, pageSize: 10) {
				items { id machineId kind performedAt printingTime { value unit } notes }
				page pageSize totalItems totalPages
			}
		}`,
		fallback,
		{ id: params.id, page }
	);

	return {
		machine: result.value.machine,
		maintenanceStatuses: result.value.machineMaintenanceStatuses,
		maintenanceHistory: result.value.machineMaintenanceHistoryFor,
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
