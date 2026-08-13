import { graphqlOrFallback } from '$lib/api/graphql';
import { Tone } from '$lib/components/ui/presets';
import type { PageLoad } from './$types';

type InvoiceResult = {
	importedInvoices: {
		id: string;
		number: string;
		customer: string;
		activityId: string | null;
		orderId: string | null;
		issuedOn: string;
		dueOn: string;
		totalTtc: string;
		paymentState: string;
	}[];
};

const fallback: InvoiceResult = {
	importedInvoices: [
		{
			id: 'demo-1',
			number: 'A-2026-034',
			customer: 'Maison Dune',
			activityId: null,
			orderId: null,
			issuedOn: '2026-08-05',
			dueOn: '2026-08-19',
			totalTtc: '1 240 €',
			paymentState: 'unpaid'
		},
		{
			id: 'demo-2',
			number: 'A-2026-033',
			customer: 'Bloom Bloom',
			activityId: null,
			orderId: null,
			issuedOn: '2026-08-01',
			dueOn: '2026-08-15',
			totalTtc: '842 €',
			paymentState: 'unpaid'
		},
		{
			id: 'demo-3',
			number: 'F-2026-018',
			customer: 'Atelier Libre',
			activityId: null,
			orderId: null,
			issuedOn: '2026-07-28',
			dueOn: '2026-07-28',
			totalTtc: '3 120 €',
			paymentState: 'paid'
		}
	]
};

export const load: PageLoad = async ({ fetch, parent }) => {
	const { defaultPageSize } = await parent();
	const result = await graphqlOrFallback<InvoiceResult>(
		fetch,
		`query InvoicesPage($pageSize: Int!) {
		importedInvoices(pageSize: $pageSize) { id number customer activityId orderId issuedOn dueOn totalTtc paymentState }
	}`,
		fallback,
		{ pageSize: defaultPageSize }
	);
	const today = new Date().toISOString().slice(0, 10);
	const invoices = result.value.importedInvoices.map((invoice) => {
		const overdue = invoice.paymentState === 'unpaid' && invoice.dueOn < today;
		return {
			id: invoice.id,
			no: invoice.number,
			customer: invoice.customer,
			activity: invoice.activityId ? 'Activité liée' : 'Non liée',
			date: invoice.issuedOn,
			due: invoice.dueOn,
			total: invoice.totalTtc,
			state: invoice.paymentState === 'paid' ? 'Payée' : overdue ? 'En retard' : 'À échéance',
			tone: invoice.paymentState === 'paid' ? Tone.Success : overdue ? Tone.Error : Tone.Warning,
			link: invoice.orderId ? 'Commande liée' : '—'
		};
	});

	return {
		invoices,
		kpis: {
			unlinked: result.value.importedInvoices.filter(
				(invoice) => !invoice.activityId && !invoice.orderId
			).length,
			open: result.value.importedInvoices.filter((invoice) => invoice.paymentState === 'unpaid')
				.length,
			overdue: invoices.filter((invoice) => invoice.state === 'En retard').length
		},
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
