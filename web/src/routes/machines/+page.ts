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

type MachinesResult = { machineModels: MachineModel[]; machines: Machine[] };

const fallback: MachinesResult = { machineModels: [], machines: [] };

export const load: PageLoad = async ({ fetch, parent }) => {
	const { defaultPageSize } = await parent();
	const result = await graphqlOrFallback<MachinesResult>(
		fetch,
		`query MachinesPage($pageSize: Int!) {
			machineModels(pageSize: $pageSize) {
				id brand name
				maintenanceCost { value numeratorUnit denominatorUnit }
				lifetime { value unit }
				averagePower { value unit }
			}
			machines(pageSize: $pageSize) {
				id surname modelId purchaseCost { value unit } printingTime { value unit } state
				model {
					id brand name
					maintenanceCost { value numeratorUnit denominatorUnit }
					lifetime { value unit }
					averagePower { value unit }
				}
			}
		}`,
		fallback,
		{ pageSize: defaultPageSize }
	);

	return {
		models: result.value.machineModels,
		machines: result.value.machines,
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
