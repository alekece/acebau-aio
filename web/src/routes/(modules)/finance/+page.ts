import { emptyPage, graphqlOrFallback, type Page } from '$lib/api/graphql';
import type { PageLoad } from './$types';

type FinanceResult = {
	expenses: Page<{ accountingDate: string; totalHt: string; totalVat: string; totalTtc: string }>;
	importedInvoices: Page<{
		issuedOn: string;
		totalHt: string;
		totalVat: string;
		paymentState: string;
	}>;
};

const fallback: FinanceResult = { expenses: emptyPage(), importedInvoices: emptyPage() };

export const load: PageLoad = async ({ fetch, parent }) => {
	const { defaultPageSize } = await parent();
	const result = await graphqlOrFallback<FinanceResult>(
		fetch,
		`query FinancePage($pageSize: Int!) {
		expenses(pageSize: $pageSize) { items { accountingDate totalHt totalVat totalTtc } }
		importedInvoices(pageSize: $pageSize) { items { issuedOn totalHt totalVat paymentState } }
	}`,
		fallback,
		{ pageSize: defaultPageSize }
	);

	return {
		recordCounts: {
			expenses: result.value.expenses.totalItems,
			invoices: result.value.importedInvoices.totalItems,
			unpaidInvoices: result.value.importedInvoices.items.filter(
				(invoice) => invoice.paymentState === 'unpaid'
			).length
		},
		// Consolidated monetary aggregates will move to the analytics/finance query service.
		summary: {
			turnover: '7 180 €',
			collected: '6 380 €',
			expenses: '2 420 €',
			result: '3 960 €',
			provision: '1 180 €',
			cash: '9 629 €',
			grossCash: '14 100 €',
			activities: [
				{ name: 'Acebau', value: '5 940 €', percentage: '83 %' },
				{ name: 'Freelance', value: '1 240 €', percentage: '17 %' }
			],
			months: [
				['Août 2026', '7 180 €', '2 420 €', '4 760 €'],
				['Juillet 2026', '8 420 €', '3 010 €', '5 410 €'],
				['Juin 2026', '6 980 €', '2 740 €', '4 240 €']
			]
		},
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
