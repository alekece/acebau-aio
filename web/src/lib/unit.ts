import Decimal from 'decimal.js';

export type LengthUnit = 'mm' | 'cm' | 'm';
export type MassUnit = 'g' | 'kg';
export type PercentageUnit = '%';
export type PowerUnit = 'W' | 'kW';
export type PriceUnit = '€';
export type TimeUnit = 'min' | 'h' | 'd' | 'y';

export type Unit = LengthUnit | MassUnit | PercentageUnit | PowerUnit | PriceUnit | TimeUnit;

/** A metric as serialized by the GraphQL API. Decimal values remain strings to preserve precision. */
export type MetricDTO<T extends Unit = Unit> = {
	value: string;
	unit: T;
};

/** A ratio as serialized by the GraphQL API. Decimal values remain strings to preserve precision. */
export type RatioDTO<Numerator extends Unit = Unit, Denominator extends Unit = Unit> = {
	value: string;
	numeratorUnit: Numerator;
	denominatorUnit: Denominator;
};

export class Metric<T extends Unit = Unit> {
	private constructor(
		readonly value: Decimal,
		readonly unit: T
	) {}

	static from<T extends Unit>(metric: MetricDTO<T>): Metric<T> {
		return new Metric(new Decimal(metric.value), metric.unit);
	}

	toDTO(): MetricDTO<T> {
		return { value: this.value.toString(), unit: this.unit };
	}
}

export class Ratio<Numerator extends Unit = Unit, Denominator extends Unit = Unit> {
	private constructor(
		readonly value: Decimal,
		readonly numeratorUnit: Numerator,
		readonly denominatorUnit: Denominator
	) {}

	static from<Numerator extends Unit, Denominator extends Unit>(
		ratio: RatioDTO<Numerator, Denominator>
	): Ratio<Numerator, Denominator> {
		return new Ratio(new Decimal(ratio.value), ratio.numeratorUnit, ratio.denominatorUnit);
	}

	toDTO(): RatioDTO<Numerator, Denominator> {
		return {
			value: this.value.toString(),
			numeratorUnit: this.numeratorUnit,
			denominatorUnit: this.denominatorUnit
		};
	}
}
