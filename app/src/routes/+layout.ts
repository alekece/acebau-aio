import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async () => {
	return {
		links: [
			{ slug: '/', title: 'Home' },
			{ slug: '/about', title: 'About' }
		]
	};
};
