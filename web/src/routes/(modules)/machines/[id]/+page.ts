import { graphqlOrFallback } from '$lib/api/graphql';
import type { MetricDTO, PowerUnit, PriceUnit, RatioDTO, TimeUnit } from '$lib/unit';
import type { PageLoad } from './$types';

type MachineDetail = {
	id: string;
	surname: string;
	modelId: string;
	purchaseCost: MetricDTO<PriceUnit>;
	printingTime: MetricDTO<TimeUnit>;
	usageCost: RatioDTO<PriceUnit, TimeUnit>;
	state: 'available' | 'running' | 'maintenance' | 'broken';
	model: {
		id: string;
		brand: string;
		name: string;
		maintenanceCost: RatioDTO<PriceUnit, TimeUnit>;
		lifetime: MetricDTO<TimeUnit>;
		averagePower: MetricDTO<PowerUnit>;
	};
};

export const load: PageLoad = async ({ fetch, params }) => {
	const fallback: { machine: MachineDetail } = {
		machine: {
			id: params.id,
			surname: 'Machine indisponible',
			modelId: '',
			purchaseCost: { value: '0', unit: '€' },
			printingTime: { value: '0', unit: 'h' },
			usageCost: { value: '0', numeratorUnit: '€', denominatorUnit: 'h' },
			state: 'available',
			model: {
				id: '',
				brand: '—',
				name: 'Modèle indisponible',
				maintenanceCost: { value: '0', numeratorUnit: '€', denominatorUnit: 'h' },
				lifetime: { value: '0', unit: 'h' },
				averagePower: { value: '0', unit: 'W' }
			}
		}
	};
	const result = await graphqlOrFallback<{ machine: MachineDetail }>(
		fetch,
		`query MachineDetail($id: String!) {
			machine(id: $id) {
				id surname modelId purchaseCost { value unit } printingTime { value unit }
				usageCost { value numeratorUnit denominatorUnit } state
				model {
					id brand name
					maintenanceCost { value numeratorUnit denominatorUnit }
					lifetime { value unit }
					averagePower { value unit }
				}
			}
		}`,
		fallback,
		{ id: params.id }
	);

	return {
		machine: result.value.machine,
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
