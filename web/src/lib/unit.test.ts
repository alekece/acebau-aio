import Decimal from 'decimal.js';
import { describe, expect, it } from 'vitest';

import { Metric, Ratio } from './unit';

describe('unit DTO conversions', () => {
	it('converts a metric without passing through a JavaScript number', () => {
		const dto = { value: '0.1234567890123456789012345678', unit: '€' } as const;
		const metric = Metric.from(dto);

		expect(metric.value.equals(new Decimal(dto.value))).toBe(true);
		expect(metric.toDTO()).toEqual(dto);
	});

	it('preserves ratio unit types and decimal values', () => {
		const dto = {
			value: '0.0833333333333333333333333333',
			numeratorUnit: '€',
			denominatorUnit: 'min'
		} as const;
		const ratio = Ratio.from(dto);

		expect(ratio.value.equals(new Decimal(dto.value))).toBe(true);
		expect(ratio.toDTO()).toEqual(dto);
	});
});
