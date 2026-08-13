type Mapper<T> = T | ((value: unknown) => T);
type Candidate<T> = [propName: string, mapper?: Mapper<T>];

type ExclusiveGroup<T> = {
	fallback?: T;
	candidates: Candidate<T>[];
};

function normalizeExclusiveGroup<T>(
	component: string,
	groupName: string,
	exclusiveGroup: ExclusiveGroup<T>,
	props: Record<string, unknown>
): T | undefined {
	const usedProps = exclusiveGroup.candidates
		.map(([name, mapper]) => [name, props[name], mapper] as const)
		.filter(([, value]) => value !== undefined && value !== false);

	if (usedProps.length > 1) {
		const names = usedProps.map(([name]) => name).join(', ');
		throw new Error(
			`${component}: props [${names}] are mutually exclusive. Use only one ${groupName} prop.`
		);
	}

	if (usedProps.length === 0) return exclusiveGroup.fallback;

	const [, value, mapper] = usedProps[0];
	if (mapper !== undefined) {
		return typeof mapper === 'function' ? (mapper as (value: unknown) => T)(value) : mapper;
	}
	if (value === true) {
		throw new Error(`${component}: shorthand requires a mapper to normalize into ${groupName}.`);
	}
	return value as T;
}

export function consumeExclusiveGroups<Specs extends Record<string, ExclusiveGroup<unknown>>>(
	component: string,
	props: Record<string, unknown>,
	specs: Specs
): {
	normalizedProps: {
		[K in keyof Specs]: Specs[K] extends ExclusiveGroup<infer Value> ? Value | undefined : never;
	};
	rest: Record<string, unknown>;
} {
	const normalizedProps: Record<string, unknown> = {};
	const excludedProps = new Set<string>();

	for (const groupName in specs) {
		const group = specs[groupName];
		normalizedProps[groupName] = normalizeExclusiveGroup(component, groupName, group, props);
		group.candidates.forEach(([name]) => excludedProps.add(name));
	}

	return {
		normalizedProps: normalizedProps as {
			[K in keyof Specs]: Specs[K] extends ExclusiveGroup<infer Value> ? Value | undefined : never;
		},
		rest: Object.fromEntries(Object.entries(props).filter(([name]) => !excludedProps.has(name)))
	};
}

/*
 * @enum Size
 * @description This enum defines the size for components.
 */
export enum Size {
	Small = 'sm',
	Medium = 'base',
	Large = 'lg'
}

export const sizeGroup: ExclusiveGroup<Size> = {
	fallback: Size.Medium,
	candidates: [
		['size', (v) => v as Size],
		['small', Size.Small],
		['medium', Size.Medium],
		['large', Size.Large]
	]
};

/*
 * @enum Tone
 * @description This enum defines the tone for components.
 */
export enum Tone {
	Primary = 'primary',
	Secondary = 'secondary',
	Tertiary = 'tertiary',
	Success = 'success',
	Warning = 'warning',
	Error = 'error',
	Surface = 'surface'
}

export const toneGroup: ExclusiveGroup<Tone> = {
	candidates: [
		['tone', (v) => v as Tone],
		['primary', Tone.Primary],
		['secondary', Tone.Secondary],
		['tertiary', Tone.Tertiary],
		['success', Tone.Success],
		['warning', Tone.Warning],
		['error', Tone.Error],
		['surface', Tone.Surface]
	]
};

/*
 * @enum Variant
 * @description This enum defines the variant for components.
 */
export enum Variant {
	Filled = 'filled',
	Tonal = 'tonal',
	Outlined = 'outlined'
}

export const variantGroup: ExclusiveGroup<Variant> = {
	fallback: Variant.Filled,
	candidates: [
		['variant', (v) => v as Variant],
		['filled', Variant.Filled],
		['tonal', Variant.Tonal],
		['outlined', Variant.Outlined]
	]
};

export const presetGroups = {
	size: sizeGroup,
	variant: variantGroup,
	tone: toneGroup
};
