<script lang="ts">
	import ChevronRight from '@lucide/svelte/icons/chevron-right';
	import DataSourceNotice from '$lib/components/ui/DataSourceNotice.svelte';
	import MachineStatus from '$lib/components/machines/MachineStatus.svelte';
	import MaintenanceCriticity from '$lib/components/machines/MaintenanceCriticity.svelte';
	import MachineMaintenanceHistory from '$lib/components/machines/MachineMaintenanceHistory.svelte';
	import ModuleHeader from '$lib/components/ui/ModuleHeader.svelte';
	import PageShell from '$lib/components/ui/PageShell.svelte';
	import {
		Metric,
		Ratio,
		type MetricDTO,
		type PowerUnit,
		type PriceUnit,
		type RatioDTO,
		type TimeUnit,
		type Unit
	} from '$lib/unit';
	import type { PageProps } from './$types';
	import { maintenanceKindLabels } from '$lib/machine';

	let { data }: PageProps = $props();

	function metricLabel(metric: MetricDTO<Unit>) {
		return `${metric.value} ${metric.unit}`;
	}

	function nozzleLabel(nozzleSize: MetricDTO<Unit>) {
		const millimeters = Metric.from(nozzleSize).convertTo('mm').value.toFixed(1).replace('.', ',');
		return `${millimeters} mm`;
	}

	function ratioLabel(ratio: RatioDTO<PriceUnit, TimeUnit>) {
		const yearly = Ratio.from(ratio).convertTo('€', 'y');
		return `${yearly.value.toDecimalPlaces(2).toString()} €/année`;
	}

	function lifetimeLabel(lifetime: MetricDTO<TimeUnit>) {
		const yearly = Metric.from(lifetime).convertTo('y');
		return `${yearly.value.toDecimalPlaces(2).toString()} ans`;
	}

	function powerLabel(power: MetricDTO<PowerUnit>) {
		const watts = Metric.from(power).convertTo('W');
		return `${watts.value.toDecimalPlaces(2).toString()} W`;
	}

	function usageCostLabel(cost: RatioDTO<PriceUnit, TimeUnit>) {
		const hourly = Ratio.from(cost).convertTo('€', 'h');
		return `${hourly.value.toDecimalPlaces(4).toString()} €/h`;
	}

	function hours(metric: MetricDTO<TimeUnit>) {
		return Metric.from(metric).convertTo('h').value.toDecimalPlaces(1).toString();
	}

	function formatDate(value: string) {
		return new Intl.DateTimeFormat('fr-FR', { dateStyle: 'medium' }).format(new Date(value));
	}
</script>

<svelte:head><title>{data.machine.surname} — Machines — Acebau</title></svelte:head>

<PageShell class="max-w-[1120px]">
	<DataSourceNotice visible={data.usingFallback} message={data.loadError} />
	<ModuleHeader
		title={`Machines > ${data.machine.surname}`}
		description={`${data.machine.model.brand} · ${data.machine.model.name} · #${data.machine.id.slice(0, 8)}`}
	>
		{#snippet heading()}
			<span class="inline-flex flex-wrap items-center gap-2">
				<a
					href="/machines"
					class="cursor-pointer text-surface-700-300 transition-colors hover:text-surface-950-50"
					>Machines</a
				>
				<ChevronRight class="text-surface-700-300 opacity-40" size={24} aria-hidden="true" />
				<span>{data.machine.surname}</span>
			</span>
		{/snippet}
		{#snippet actions()}<MachineStatus state={data.machine.state} />{/snippet}
	</ModuleHeader>

	<div class="flex flex-col gap-6">
		<section class="overflow-hidden card border border-surface-300-700 bg-surface-50-950 shadow-sm">
			<div class="p-6">
				<h2 class="mb-5 text-lg font-bold text-surface-900-100">Machine</h2>
				<dl class="grid grid-cols-2 gap-x-6 gap-y-5 max-[520px]:grid-cols-1">
					<div>
						<dt class="text-sm text-surface-700-300">Prix d’achat</dt>
						<dd class="mt-1 font-semibold">{metricLabel(data.machine.purchaseCost)}</dd>
					</div>
					<div>
						<dt class="text-sm text-surface-700-300">Buse actuelle</dt>
						<dd class="mt-1 font-semibold">{nozzleLabel(data.machine.nozzleSize)}</dd>
					</div>
					<div>
						<dt class="text-sm text-surface-700-300">Temps d’impression enregistré</dt>
						<dd class="mt-1 font-semibold">{metricLabel(data.machine.printingTime)}</dd>
					</div>
					<div>
						<dt class="text-sm text-surface-700-300">Coût d’usage</dt>
						<dd class="mt-1 font-semibold">{usageCostLabel(data.machine.usageCost)}</dd>
					</div>
					<div>
						<dt class="text-sm text-surface-700-300">Identifiant</dt>
						<dd class="mt-1 font-mono text-sm break-all">{data.machine.id}</dd>
					</div>
				</dl>
			</div>

			<div class="border-t border-surface-300-700 p-6">
				<h2 class="mb-5 text-lg font-bold text-surface-900-100">Modèle</h2>
				<dl class="grid grid-cols-2 gap-x-6 gap-y-5 max-[520px]:grid-cols-1">
					<div>
						<dt class="text-sm text-surface-700-300">Fabricant</dt>
						<dd class="mt-1 font-semibold">{data.machine.model.brand}</dd>
					</div>
					<div>
						<dt class="text-sm text-surface-700-300">Modèle</dt>
						<dd class="mt-1 font-semibold">{data.machine.model.name}</dd>
					</div>
					<div>
						<dt class="text-sm text-surface-700-300">Puissance moyenne</dt>
						<dd class="mt-1 font-semibold">{powerLabel(data.machine.model.averagePower)}</dd>
					</div>
					<div>
						<dt class="text-sm text-surface-700-300">Durée de vie</dt>
						<dd class="mt-1 font-semibold">{lifetimeLabel(data.machine.model.lifetime)}</dd>
					</div>
					<div class="col-span-2 max-[520px]:col-span-1">
						<dt class="text-sm text-surface-700-300">Coût de maintenance</dt>
						<dd class="mt-1 font-semibold">{ratioLabel(data.machine.model.maintenanceCost)}</dd>
					</div>
				</dl>
			</div>
		</section>

		<section class="card border border-surface-300-700 bg-surface-50-950 p-6 shadow-sm">
			<div class="mb-5">
				<h2 class="text-lg font-bold text-surface-900-100">Maintenance</h2>
				<p class="mt-1 text-sm text-surface-700-300">
					Échéances calculées depuis le compteur d’impression.
				</p>
			</div>
			<div class="grid grid-cols-2 gap-3 max-[700px]:grid-cols-1">
				{#each data.maintenanceStatuses as status (status.kind)}
					<article class="rounded-base border border-surface-300-700 bg-surface-100-900 p-4">
						<div class="flex items-start justify-between gap-3">
							<strong>{maintenanceKindLabels[status.kind]}</strong>
							<MaintenanceCriticity criticity={status.criticity} />
						</div>
						<p class="mt-3 text-sm text-surface-700-300">
							{hours(status.printingTimeSinceMaintenance)} h depuis la dernière opération · à prévoir
							à
							{hours(status.dueAfter)} h · critique à {hours(status.criticalAfter)} h
						</p>
						<small
							>{status.lastPerformedAt
								? `Dernière intervention le ${formatDate(status.lastPerformedAt)}`
								: 'Aucune intervention enregistrée'}</small
						>
					</article>
				{/each}
			</div>
			<div class="mt-6 border-t border-surface-300-700 pt-5">
				<h3 class="mb-3 font-semibold">Historique</h3>
				<MachineMaintenanceHistory records={data.maintenanceHistory} pageParam="maintenancePage" />
			</div>
		</section>
	</div>
</PageShell>
