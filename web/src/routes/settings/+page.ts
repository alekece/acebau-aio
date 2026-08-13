import { emptyPage, graphqlOrFallback, type Page } from '$lib/api/graphql';
import type { PageLoad } from './$types';

type SettingsResult = {
	applicationSettings: Page<{
		id: string;
		defaultTimeUnit: string;
		defaultMassUnit: string;
		defaultLengthUnit: string;
		defaultPowerUnit: string;
		defaultPageSize: number;
		electricityRate: string;
	}>;
};

const fallback: SettingsResult = {
	applicationSettings: emptyPage([
		{
			id: '00000000-0000-0000-0000-000000000001',
			defaultTimeUnit: 'h',
			defaultMassUnit: 'g',
			defaultLengthUnit: 'mm',
			defaultPowerUnit: 'W',
			defaultPageSize: 10,
			electricityRate: '0.25€/kWh'
		}
	])
};

export const load: PageLoad = async ({ fetch }) => {
	const result = await graphqlOrFallback<SettingsResult>(
		fetch,
		`query SettingsPage {
		applicationSettings { items { id defaultTimeUnit defaultMassUnit defaultLengthUnit defaultPowerUnit defaultPageSize electricityRate } }
	}`,
		fallback
	);

	return {
		settings: result.value.applicationSettings.items[0],
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
