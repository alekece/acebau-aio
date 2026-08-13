<script lang="ts">
	import Table from '$lib/components/ui/Table.svelte';
	import { maintenanceKindLabels, type MachineMaintenancePage } from '$lib/machine';
	import { Metric } from '$lib/unit';

	let {
		records,
		showMachine = false,
		page,
		pageSize,
		totalItems,
		totalPages,
		onPageChange,
		pageParam = 'page'
	}: {
		records: MachineMaintenancePage;
		showMachine?: boolean;
		page?: number;
		pageSize?: number;
		totalItems?: number;
		totalPages?: number;
		onPageChange?: (page: number) => void | Promise<void>;
		pageParam?: string;
	} = $props();

	let items = $derived(records.items);

	function formatDate(value: string) {
		return new Intl.DateTimeFormat('fr-FR', { dateStyle: 'medium' }).format(new Date(value));
	}
</script>

<Table
	responsiveCards
	class={showMachine ? 'min-w-[52rem]' : 'min-w-[38rem]'}
	page={page ?? records.page}
	pageSize={pageSize ?? records.pageSize}
	totalItems={totalItems ?? records.totalItems}
	totalPages={totalPages ?? records.totalPages}
	{onPageChange}
	{pageParam}
>
	<thead>
		<tr>
			{#if showMachine}<th>Machine</th>{/if}
			<th>Opération</th><th>Date</th><th>Compteur</th><th>Notes</th>
		</tr>
	</thead>
	<tbody>
		{#each items as maintenance (maintenance.id)}
			<tr>
				{#if showMachine}<td data-label="Machine"
						><strong>{maintenance.machine?.surname ?? '—'}</strong></td
					>{/if}
				<td data-label="Opération">{maintenanceKindLabels[maintenance.kind]}</td>
				<td data-label="Date">{formatDate(maintenance.performedAt)}</td>
				<td data-label="Compteur"
					>{Metric.from(maintenance.printingTime).convertTo('h').value.toDecimalPlaces(1)} h</td
				>
				<td data-label="Notes">{maintenance.notes || '—'}</td>
			</tr>
		{:else}
			<tr
				><td colspan={showMachine ? 5 : 4} class="py-10 text-center text-surface-700-300"
					>Aucune maintenance enregistrée.</td
				></tr
			>
		{/each}
	</tbody>
</Table>
