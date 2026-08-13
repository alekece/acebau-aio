<script lang="ts">
	import MaintenanceCriticity from '$lib/components/machines/MaintenanceCriticity.svelte';
	import { maintenanceKindLabels, type MachineMaintenanceStatus } from '$lib/machine';
	import { Metric } from '$lib/unit';

	type MachineSummary = { id: string; surname: string };

	let {
		machines,
		statuses
	}: {
		machines: MachineSummary[];
		statuses: Record<string, MachineMaintenanceStatus[]>;
	} = $props();

	function nearest(machineId: string) {
		return [...(statuses[machineId] ?? [])].sort((left, right) => {
			const priority = { critical: 0, due: 1, normal: 2 } as const;
			if (priority[left.criticity] !== priority[right.criticity]) {
				return priority[left.criticity] - priority[right.criticity];
			}
			const leftRemaining = Metric.from(left.dueAfter)
				.convertTo('h')
				.value.minus(Metric.from(left.printingTimeSinceMaintenance).convertTo('h').value);
			const rightRemaining = Metric.from(right.dueAfter)
				.convertTo('h')
				.value.minus(Metric.from(right.printingTimeSinceMaintenance).convertTo('h').value);
			return leftRemaining.comparedTo(rightRemaining);
		})[0];
	}

	function detail(status: MachineMaintenanceStatus) {
		if (status.criticity === 'critical') return 'Limite dépassée';
		if (status.criticity === 'due') return 'À effectuer';
		const remaining = Metric.from(status.dueAfter)
			.convertTo('h')
			.value.minus(Metric.from(status.printingTimeSinceMaintenance).convertTo('h').value);
		return `Dans ${remaining.toDecimalPlaces(0)} h`;
	}
</script>

<section
	class="overflow-hidden rounded-container border border-surface-300-700 bg-surface-50-950 shadow-sm"
>
	<header class="border-b border-surface-300-700 p-5">
		<h2 class="m-0 text-xl font-bold text-surface-900-100">Prochaines maintenances</h2>
		<p class="mt-1 mb-0 text-sm text-surface-700-300">
			L’opération la plus proche pour chaque machine.
		</p>
	</header>
	<div class="divide-y divide-surface-300-700">
		{#each machines as machine (machine.id)}
			{@const status = nearest(machine.id)}
			<div class="flex items-center justify-between gap-4 px-5 py-4">
				<div class="min-w-0">
					<strong class="block truncate">{machine.surname}</strong>
					<small class="text-surface-700-300">
						{status
							? `${maintenanceKindLabels[status.kind]} · ${detail(status)}`
							: 'Non configurée'}
					</small>
				</div>
				{#if status}<MaintenanceCriticity criticity={status.criticity} />{/if}
			</div>
		{/each}
	</div>
</section>
