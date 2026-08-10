import { error } from '@sveltejs/kit';
import { Tone } from '$lib/types';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, parent }) => {
	const data = await parent();
	const variant = data.variants.find((candidate) => candidate.id === params.variantId);
	if (!variant) error(404, 'Variante introuvable');

	return {
		variant,
		stock: { available: '—', toProduce: '—', defective: '—' },
		margins: [
			{ label: 'Revendeur', value: '—', tone: Tone.Surface },
			{ label: 'Site', value: '—', tone: Tone.Surface },
			{ label: 'Direct', value: '—', tone: Tone.Surface }
		]
	};
};
