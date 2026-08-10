import { graphqlOrFallback } from '$lib/api/graphql';
import type { PageLoad } from './$types';

type MachineModel = {
	id: string;
	brand: string;
	name: string;
	purchaseCost: string;
	maintenanceCost: string;
	lifetime: string;
	averagePower: string;
};

type Machine = {
	id: string;
	surname: string;
	modelId: string;
	printingTime: string;
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
			machineModels(pageSize: $pageSize) { id brand name purchaseCost maintenanceCost lifetime averagePower }
			machines(pageSize: $pageSize) {
				id surname modelId printingTime state
				model { id brand name purchaseCost maintenanceCost lifetime averagePower }
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
