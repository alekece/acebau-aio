import { graphqlOrFallback } from '$lib/api/graphql';
import type { MetricDTO, PowerUnit, PriceUnit, RatioDTO, TimeUnit } from '$lib/unit';
import type { PageLoad } from './$types';

type MachineModel = {
	id: string;
	brand: string;
	name: string;
	maintenanceCost: RatioDTO<PriceUnit, TimeUnit>;
	lifetime: MetricDTO<TimeUnit>;
	averagePower: MetricDTO<PowerUnit>;
};

type Machine = {
	id: string;
	surname: string;
	modelId: string;
	purchaseCost: MetricDTO<PriceUnit>;
	printingTime: MetricDTO<TimeUnit>;
	state: 'available' | 'running' | 'maintenance' | 'broken';
	model: MachineModel;
};

type MachinePage = {
	items: Machine[];
	page: number;
	pageSize: number;
	totalItems: number;
	totalPages: number;
};

type MachinesResult = { machineModels: { items: MachineModel[] }; machines: MachinePage };

const fallback: MachinesResult = {
	machineModels: { items: [] },
	machines: { items: [], page: 1, pageSize: 10, totalItems: 0, totalPages: 0 }
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
				}
			}
			machines(page: $page, pageSize: $pageSize) {
				items {
					id surname modelId purchaseCost { value unit } printingTime { value unit } state
					model {
						id brand name
						maintenanceCost { value numeratorUnit denominatorUnit }
						lifetime { value unit }
						averagePower { value unit }
					}
				}
				page pageSize totalItems totalPages
			}
		}`,
		fallback,
		{ page: 1, pageSize: 10 }
	);

	return {
		models: result.value.machineModels.items,
		machines: result.value.machines.items,
		machinePage: result.value.machines,
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
