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

	it('converts metrics through canonical factors', () => {
		const time = Metric.from({ value: '60', unit: 'min' as const });
		const mass = Metric.from({ value: '1', unit: 'kg' as const });

		expect(time.convertTo('h').toDTO()).toEqual({ value: '1', unit: 'h' });
		expect(mass.convertTo('g').toDTO()).toEqual({ value: '1000', unit: 'g' });
		expect(Metric.from({ value: '12', unit: 'mo' as const }).convertTo('y').toDTO()).toEqual({
			value: '1',
			unit: 'y'
		});
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

	it('converts ratio numerator and denominator units', () => {
		const hourly = Ratio.from({
			value: '5',
			numeratorUnit: '€' as const,
			denominatorUnit: 'h' as const
		});
		const perMinute = hourly.convertTo('€', 'min');

		expect(perMinute.value.toSignificantDigits(15).toString()).toBe('0.0833333333333333');
		expect(perMinute.numeratorUnit).toBe('€');
		expect(perMinute.denominatorUnit).toBe('min');
	});

	it('rejects conversion between different dimensions', () => {
		const metric = Metric.from({ value: '1', unit: 'h' as const });

		expect(() => metric.convertTo('kg' as never)).toThrow('Cannot convert h to kg');
	});
});
