import { emptyPage, graphqlOrFallback, type Page } from '$lib/api/graphql';
import { FALLBACK_METRIC_DEFAULTS } from '$lib/settings/metric-defaults';
import type { LayoutLoad } from './$types';

type SettingsResult = {
	applicationSettings: Page<{
		defaultTimeUnit: string;
		defaultMassUnit: string;
		defaultLengthUnit: string;
		defaultPowerUnit: string;
		defaultPageSize: number;
	}>;
};

export const load: LayoutLoad = async ({ fetch }) => {
	const result = await graphqlOrFallback<SettingsResult>(
		fetch,
		`query LayoutSettings {
			applicationSettings {
				items { defaultTimeUnit defaultMassUnit defaultLengthUnit defaultPowerUnit defaultPageSize }
			}
		}`,
		{
			applicationSettings: emptyPage([
				{
					defaultTimeUnit: FALLBACK_METRIC_DEFAULTS.time,
					defaultMassUnit: FALLBACK_METRIC_DEFAULTS.mass,
					defaultLengthUnit: FALLBACK_METRIC_DEFAULTS.length,
					defaultPowerUnit: FALLBACK_METRIC_DEFAULTS.power,
					defaultPageSize: 10
				}
			])
		}
	);
	const settings = result.value.applicationSettings.items[0];

	return {
		metricDefaults: settings
			? {
					time: settings.defaultTimeUnit,
					mass: settings.defaultMassUnit,
					length: settings.defaultLengthUnit,
					power: settings.defaultPowerUnit
				}
			: FALLBACK_METRIC_DEFAULTS,
		defaultPageSize: settings?.defaultPageSize ?? 10
	};
};
