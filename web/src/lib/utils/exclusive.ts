export type Mapper<T> = T | ((value: unknown) => T);
type Candidate<T> = [propName: string, mapper?: Mapper<T>];

export type ExclusiveGroup<T> = {
	fallback?: T,
	candidates: Candidate<T>[];
};

function normalizeExclusiveGroup<T>(
	component: string,
	groupName: string,
	exclusiveGroup: ExclusiveGroup<T>,
	props: Record<string, unknown>
): T {
	const usedProp = exclusiveGroup.candidates
		.map(([k, m]) => [k, props[k], m] as [string, unknown, Mapper<T> | undefined])
		.filter(([, v]) => v !== undefined && v !== false);

	if (usedProp.length > 1) {
		const names = usedProp.map(([k]) => k).join(', ');
		throw new Error(
			`${component}: props [${names}] are mutually exclusive. Use only one ${groupName} prop.`
		);
	}

	if (usedProp.length === 0) return exclusiveGroup.fallback;

	const [, value, mapper] = usedProp[0];

	if (mapper !== undefined) {
		return typeof mapper === 'function' ? (mapper as (v: unknown) => T)(value) : (mapper as T);
	}

	if (value === true) {
		throw new Error(`${component}: shorthand requires a mapper to normalize into ${groupName}.`);
	}

	return value as T;
}

export function consumeExclusiveGroups<Specs extends Record<string, ExclusiveGroup<any>>>(
	component: string,
	props: Record<string, unknown>,
	specs: Specs
): {
	normalizedProps: { [K in keyof Specs]: Specs[K]['fallback'] };
	rest: Record<string, any>;
} {
	const normalizedProps: Record<string, unknown> = {};
	const excludedProps = new Set<string>();

	for (const groupName in specs) {
		const exclusiveGroup = specs[groupName];

		normalizedProps[groupName] = normalizeExclusiveGroup(component, groupName, exclusiveGroup, props);
		exclusiveGroup.candidates.forEach(([name]) => excludedProps.add(name));
	}

	const rest: Record<string, unknown> = {};
	for (const [k, v] of Object.entries(props)) {
		if (!excludedProps.has(k)) rest[k] = v;
	}
	return { normalizedProps: normalizedProps as any, rest };
}
