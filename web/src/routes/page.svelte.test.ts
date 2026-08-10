import { page } from '@vitest/browser/context';
import { describe, expect, it } from 'vitest';
import { render } from 'vitest-browser-svelte';
import Page from './+page.svelte';

describe('/+page.svelte', () => {
	it('should render h1', async () => {
		render(Page, {
			params: {},
			data: {
				actions: [],
				summaries: [],
				workshop: [],
				metricDefaults: { time: 'h', mass: 'g', length: 'mm', power: 'W' },
				defaultPageSize: 10
			}
		});

		const heading = page.getByRole('heading', { level: 1 });
		await expect.element(heading).toBeInTheDocument();
	});
});
