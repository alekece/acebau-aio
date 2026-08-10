import { getContext } from 'svelte';

export type MetricKind = 'time' | 'mass' | 'length' | 'power';

export type MetricDefaults = Record<MetricKind, string>;

export const FALLBACK_METRIC_DEFAULTS: MetricDefaults = {
	time: 'h',
	mass: 'g',
	length: 'mm',
	power: 'W'
};

export const METRIC_DEFAULTS_CONTEXT = Symbol('acebau.metric-defaults');

export function getMetricDefaults(): () => MetricDefaults {
	return (
		getContext<() => MetricDefaults>(METRIC_DEFAULTS_CONTEXT) ?? (() => FALLBACK_METRIC_DEFAULTS)
	);
}
