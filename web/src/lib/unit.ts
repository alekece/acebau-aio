import Decimal from 'decimal.js';

const UnitDecimal = Decimal.clone({ precision: 50 });

export type LengthUnit = 'mm' | 'cm' | 'm';
export type MassUnit = 'g' | 'kg';
export type PercentageUnit = '%';
export type PowerUnit = 'W' | 'kW';
export type PriceUnit = '€';
export type TimeUnit = 'min' | 'h' | 'd' | 'mo' | 'y';

export type Unit = LengthUnit | MassUnit | PercentageUnit | PowerUnit | PriceUnit | TimeUnit;

type UnitFamily<T extends Unit> = T extends LengthUnit
	? LengthUnit
	: T extends MassUnit
		? MassUnit
		: T extends PercentageUnit
			? PercentageUnit
			: T extends PowerUnit
				? PowerUnit
				: T extends PriceUnit
					? PriceUnit
					: TimeUnit;

type Dimension = 'length' | 'mass' | 'percentage' | 'power' | 'price' | 'time';

const unitDefinitions: Record<Unit, { dimension: Dimension; factor: Decimal }> = {
	mm: { dimension: 'length', factor: new UnitDecimal(1) },
	cm: { dimension: 'length', factor: new UnitDecimal(10) },
	m: { dimension: 'length', factor: new UnitDecimal(1000) },
	g: { dimension: 'mass', factor: new UnitDecimal(1) },
	kg: { dimension: 'mass', factor: new UnitDecimal(1000) },
	'%': { dimension: 'percentage', factor: new UnitDecimal(1) },
	W: { dimension: 'power', factor: new UnitDecimal(1) },
	kW: { dimension: 'power', factor: new UnitDecimal(1000) },
	'€': { dimension: 'price', factor: new UnitDecimal(1) },
	min: { dimension: 'time', factor: new UnitDecimal(1) },
	h: { dimension: 'time', factor: new UnitDecimal(60) },
	d: { dimension: 'time', factor: new UnitDecimal(1440) },
	mo: { dimension: 'time', factor: new UnitDecimal(43800) },
	y: { dimension: 'time', factor: new UnitDecimal(525600) }
};

function conversionFactor(from: Unit, to: Unit): Decimal {
	const source = unitDefinitions[from];
	const target = unitDefinitions[to];
	if (source.dimension !== target.dimension) {
		throw new Error(`Cannot convert ${from} to ${to}`);
	}
	return source.factor.dividedBy(target.factor);
}

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

	static from<T extends Unit>(metric: MetricDTO<T>): Metric<UnitFamily<T>> {
		return new Metric(new UnitDecimal(metric.value), metric.unit as unknown as UnitFamily<T>);
	}

	toDTO(): MetricDTO<T> {
		return { value: this.value.toString(), unit: this.unit };
	}

	convertTo(unit: T): Metric<T> {
		return new Metric(this.value.times(conversionFactor(this.unit, unit)), unit);
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
	): Ratio<UnitFamily<Numerator>, UnitFamily<Denominator>> {
		return new Ratio(
			new UnitDecimal(ratio.value),
			ratio.numeratorUnit as unknown as UnitFamily<Numerator>,
			ratio.denominatorUnit as unknown as UnitFamily<Denominator>
		);
	}

	toDTO(): RatioDTO<Numerator, Denominator> {
		return {
			value: this.value.toString(),
			numeratorUnit: this.numeratorUnit,
			denominatorUnit: this.denominatorUnit
		};
	}

	convertTo(numeratorUnit: Numerator, denominatorUnit: Denominator): Ratio<Numerator, Denominator> {
		const numeratorFactor = conversionFactor(this.numeratorUnit, numeratorUnit);
		const denominatorFactor = conversionFactor(denominatorUnit, this.denominatorUnit);
		return new Ratio(this.value.times(numeratorFactor).times(denominatorFactor), numeratorUnit, denominatorUnit);
	}
}
