import { Tone } from '$lib/types';
import type { PageLoad } from './$types';

export const load: PageLoad = () => ({
	// Commercial aggregates are intentionally loader-owned until analytics queries are implemented.
	products: [
		['ONDRA', '42', '5 880 €', '61 %', Tone.Success],
		['Bloom', '31', '3 920 €', '58 %', Tone.Success],
		['Mistral', '11', '1 080 €', '34 %', Tone.Warning],
		['Support mural', '7', '420 €', '19 %', Tone.Error]
	]
});
