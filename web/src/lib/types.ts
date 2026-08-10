import { type ExclusiveGroup } from '$lib/utils/exclusive';

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
