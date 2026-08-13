<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import Check from '@lucide/svelte/icons/check';
	import ChevronDown from '@lucide/svelte/icons/chevron-down';
	import Pencil from '@lucide/svelte/icons/pencil';
	import Play from '@lucide/svelte/icons/play';
	import Plus from '@lucide/svelte/icons/plus';
	import RotateCcw from '@lucide/svelte/icons/rotate-ccw';
	import Save from '@lucide/svelte/icons/save';
	import Trash2 from '@lucide/svelte/icons/trash-2';
	import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
	import X from '@lucide/svelte/icons/x';
	import { SvelteSet } from 'svelte/reactivity';
	import { graphql } from '$lib/api/graphql';
	import type {
		PieceChoice,
		ProductionBundle,
		ProductionChoice,
		ProductionJob
	} from '$lib/production/types';
	import Badge from '$lib/components/ui/Badge.svelte';
	import IconAction from '$lib/components/ui/IconAction.svelte';
	import Input from '$lib/components/ui/forms/Input.svelte';

	type JobForm = {
		pieceId: string;
		filamentSupplyId: string;
		machineId: string;
		quantity: number;
	};

	let {
		task,
		expanded = false,
		pieces = [],
		filaments = [],
		machines = [],
		ontoggle
	}: {
		task: ProductionBundle;
		expanded?: boolean;
		pieces?: PieceChoice[];
		filaments?: ProductionChoice[];
		machines?: ProductionChoice[];
		ontoggle: () => void;
	} = $props();

	const busyKeys = new SvelteSet<string>();
	let error = $state('');
	let editingBundle = $state(false);
	let jobForms = $state<Record<string, JobForm>>({});
	let addingJob = $state(false);
	let failureJobId = $state<string | null>(null);
	let bundleForm = $state({
		linkedOrderReference: '',
		deadline: '',
		planningPreference: 'quality'
	});
	let newJobForm = $state<JobForm>({
		pieceId: '',
		filamentSupplyId: '',
		machineId: '',
		quantity: 1
	});
	let failureForm = $state({ reason: '', failedQuantity: 1, actualWasteGrams: 0 });

	function stop(event: Event) {
		event.stopPropagation();
	}

	function beginBundleEdit() {
		bundleForm = {
			linkedOrderReference: task.linkedOrderReference ?? '',
			deadline: task.deadline,
			planningPreference: task.planningPreference
		};
		editingBundle = true;
	}

	function beginJobEdit(job: ProductionJob) {
		jobForms[job.id] = {
			pieceId: job.pieceId,
			filamentSupplyId: job.filamentSupplyId,
			machineId: job.machineId ?? '',
			quantity: job.quantity
		};
	}

	function beginAddJob() {
		newJobForm = {
			pieceId: pieces[0]?.id ?? '',
			filamentSupplyId: filaments[0]?.id ?? '',
			machineId: '',
			quantity: 1
		};
		addingJob = true;
	}

	function cancelJobEdit(id: string) {
		delete jobForms[id];
	}

	function isBusy(key: string) {
		return busyKeys.has(key);
	}

	async function mutate(query: string, variables: Record<string, unknown>, key: string) {
		busyKeys.add(key);
		error = '';
		try {
			await graphql(fetch, query, variables);
			await invalidateAll();
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'La production ne peut pas être modifiée.';
			throw cause;
		} finally {
			busyKeys.delete(key);
		}
	}

	async function saveBundle() {
		try {
			await mutate(
				`mutation UpdateProductionBundle($id: UUID!, $input: UpdateProductionBundleInput!) {
					updateProductionBundle(id: $id, input: $input) { id }
				}`,
				{
					id: task.id,
					input: {
						linkedOrderReference: bundleForm.linkedOrderReference || null,
						deadline: bundleForm.deadline,
						planningPreference: bundleForm.planningPreference
					}
				},
				'bundle'
			);
			editingBundle = false;
		} catch {
			// The inline form stays open so the administrator can correct it.
		}
	}

	async function deleteBundle() {
		if (!confirm(`Supprimer la production ${task.reference} et tous ses passages ?`)) return;
		try {
			await mutate(
				`mutation DeleteProductionBundle($id: UUID!) { deleteProductionBundle(id: $id) }`,
				{ id: task.id },
				'bundle'
			);
		} catch {
			// Error is displayed inside the expanded bundle.
		}
	}

	function jobInput(form: JobForm) {
		return {
			pieceId: form.pieceId,
			filamentSupplyId: form.filamentSupplyId,
			machineId: form.machineId || null,
			quantity: Number(form.quantity)
		};
	}

	async function saveJob(id: string) {
		const form = jobForms[id];
		if (!form) return;
		try {
			await mutate(
				`mutation UpdateProductionJob($id: UUID!, $input: UpdateProductionJobInput!) {
					updateProductionJob(id: $id, input: $input) { id }
				}`,
				{ id, input: jobInput(form) },
				id
			);
			cancelJobEdit(id);
		} catch {
			// Error is displayed inside the expanded bundle.
		}
	}

	async function addJob() {
		try {
			await mutate(
				`mutation AddProductionJob($taskId: UUID!, $input: UpdateProductionJobInput!) {
					addProductionJob(productionTaskId: $taskId, input: $input) { id }
				}`,
				{ taskId: task.id, input: jobInput(newJobForm) },
				'new-job'
			);
			addingJob = false;
		} catch {
			// Error is displayed inside the expanded bundle.
		}
	}

	async function deleteJob(job: ProductionJob) {
		if (!confirm(`Supprimer le passage « ${job.pieceLabel} » ?`)) return;
		try {
			await mutate(
				`mutation DeleteProductionJob($id: UUID!) { deleteProductionJob(id: $id) }`,
				{ id: job.id },
				job.id
			);
		} catch {
			// Error is displayed inside the expanded bundle.
		}
	}

	async function startJob(job: ProductionJob) {
		try {
			await mutate(
				`mutation StartProductionJob($id: UUID!) { startProductionJob(id: $id) { id } }`,
				{ id: job.id },
				job.id
			);
		} catch {
			// Error is displayed inside the expanded bundle.
		}
	}

	async function completeJob(job: ProductionJob) {
		try {
			await mutate(
				`mutation CompleteProductionJob($id: UUID!) { completeProductionJob(id: $id) { id } }`,
				{ id: job.id },
				job.id
			);
		} catch {
			// Error is displayed inside the expanded bundle.
		}
	}

	function beginFailure(job: ProductionJob) {
		failureForm = { reason: '', failedQuantity: job.quantity, actualWasteGrams: 0 };
		failureJobId = job.id;
	}

	async function failJob(job: ProductionJob) {
		try {
			await mutate(
				`mutation FailProductionJob($id: UUID!, $input: FailProductionJobInput!) {
					failProductionJob(id: $id, input: $input) { id }
				}`,
				{
					id: job.id,
					input: {
						reason: failureForm.reason,
						failedQuantity: Number(failureForm.failedQuantity),
						actualWasteGrams: Number(failureForm.actualWasteGrams)
					}
				},
				job.id
			);
			failureJobId = null;
		} catch {
			// Error is displayed inside the expanded bundle.
		}
	}

	function displayDate(value: string | null) {
		if (!value) return '—';
		return new Intl.DateTimeFormat('fr-FR', { dateStyle: 'short', timeStyle: 'short' }).format(
			new Date(value)
		);
	}
</script>

<tr class="cursor-pointer" class:bg-surface-100-900={expanded} onclick={ontoggle}>
	<td data-label="" class="mobile-card-hidden w-10">
		<ChevronDown
			size={17}
			class={`transition-transform duration-200 ${expanded ? 'rotate-180' : ''}`}
		/>
	</td>
	<td data-label="Opération">
		<strong>{task.reference}</strong>
		<small>{task.scopeLabel}</small>
	</td>
	<td data-label="État"><Badge variant="tonal" tone={task.tone}>{task.stateLabel}</Badge></td>
	<td data-label="Progression">
		<span class="font-semibold">{task.completedJobs}/{task.totalJobs}</span>
		<small>passages terminés</small>
	</td>
	<td data-label="Promesse">{task.deadline}</td>
	<td data-label="" class="mobile-card-actions w-28">
		<div class="flex justify-end gap-1" onclick={stop} role="presentation">
			{#if task.state !== 'completed'}
				<IconAction
					label={`Modifier ${task.reference}`}
					onclick={() => {
						if (!expanded) ontoggle();
						beginBundleEdit();
					}}><Pencil size={15} /></IconAction
				>
				<IconAction
					label={`Supprimer ${task.reference}`}
					tone="error"
					disabled={isBusy('bundle')}
					onclick={deleteBundle}><Trash2 size={15} /></IconAction
				>
			{/if}
		</div>
	</td>
</tr>

{#if expanded}
	<tr class="bg-surface-100-900">
		<td data-label="" colspan="6" class="!p-0">
			<div class="border-y border-surface-300-700 px-5 py-4">
				{#if editingBundle}
					<div
						class="mb-4 grid grid-cols-[minmax(0,1fr)_10rem_10rem_auto] items-end gap-3 rounded-container border border-tertiary-500/30 bg-surface-50-950 p-4 max-[900px]:grid-cols-2 max-[600px]:grid-cols-1"
					>
						<label class="label"
							>Commande liée<Input bind:value={bundleForm.linkedOrderReference} /></label
						>
						<label class="label"
							>Promesse<Input type="date" bind:value={bundleForm.deadline} /></label
						>
						<label class="label"
							>Préférence<Input as="select" bind:value={bundleForm.planningPreference}
								><option value="quality">Qualité</option><option value="cost">Coût</option><option
									value="speed">Rapidité</option
								></Input
							></label
						>
						<div class="flex gap-1">
							<IconAction
								label="Enregistrer la production"
								tone="tertiary"
								disabled={isBusy('bundle')}
								onclick={saveBundle}><Save size={15} /></IconAction
							>
							<IconAction
								label="Annuler la modification"
								tone="error"
								onclick={() => (editingBundle = false)}><X size={15} /></IconAction
							>
						</div>
					</div>
				{/if}

				<div class="mb-3">
					<div>
						<p class="m-0 text-xs font-semibold tracking-wide text-surface-700-300 uppercase">
							Passages d’impression
						</p>
						<p class="mt-1 mb-0 text-xs text-surface-600-400">
							La machine doit être affectée avant de lancer un passage.
						</p>
					</div>
				</div>

				<div
					class="job-table-wrap overflow-x-auto rounded-container border border-surface-300-700 bg-surface-50-950"
				>
					<table class="job-table w-full min-w-[980px] text-left text-sm">
						<thead class="bg-surface-100-900 text-xs tracking-wide text-surface-700-300 uppercase">
							<tr>
								<th class="px-3 py-2">Pièce</th><th class="px-3 py-2">Qté</th><th class="px-3 py-2"
									>Filament</th
								><th class="px-3 py-2">Machine</th><th class="px-3 py-2">État</th><th
									class="px-3 py-2">Début</th
								><th class="px-3 py-2 text-right">Actions</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-surface-300-700">
							{#each task.jobs as job (job.id)}
								<tr>
									{#if jobForms[job.id]}
										<td class="p-2"
											><Input as="select" bind:value={jobForms[job.id].pieceId}
												>{#each pieces as piece (piece.id)}<option value={piece.id}
														>{piece.label}</option
													>{/each}</Input
											></td
										>
										<td class="w-20 p-2"
											><Input type="number" min="1" bind:value={jobForms[job.id].quantity} /></td
										>
										<td class="p-2"
											><Input as="select" bind:value={jobForms[job.id].filamentSupplyId}
												>{#each filaments as filament (filament.id)}<option value={filament.id}
														>{filament.label}</option
													>{/each}</Input
											></td
										>
										<td class="p-2"
											><Input as="select" bind:value={jobForms[job.id].machineId}
												><option value="">À affecter</option
												>{#each machines as machine (machine.id)}<option value={machine.id}
														>{machine.label}</option
													>{/each}</Input
											></td
										>
									{:else}
										<td class="px-3 py-3 font-medium">{job.pieceLabel}</td>
										<td class="px-3 py-3">{job.quantity}</td>
										<td class="px-3 py-3">{job.filamentLabel}</td>
										<td class="px-3 py-3">{job.machineLabel}</td>
									{/if}
									<td class="px-3 py-3"
										><Badge variant="tonal" tone={job.tone}>{job.stateLabel}</Badge></td
									>
									<td class="px-3 py-3 text-xs">{displayDate(job.startedAt)}</td>
									<td class="px-3 py-2">
										<div class="flex justify-end gap-1">
											{#if jobForms[job.id]}
												<IconAction
													label="Enregistrer le passage"
													tone="tertiary"
													disabled={isBusy(job.id)}
													onclick={() => saveJob(job.id)}><Save size={15} /></IconAction
												>
												<IconAction
													label="Annuler la modification"
													tone="error"
													onclick={() => cancelJobEdit(job.id)}><X size={15} /></IconAction
												>
											{:else if job.state === 'to_print' || job.state === 'failed'}
												<IconAction label="Modifier le passage" onclick={() => beginJobEdit(job)}
													><Pencil size={15} /></IconAction
												>
												<IconAction
													label={!job.machineId
														? 'Affectez une machine avant de lancer'
														: job.state === 'failed'
															? 'Relancer le passage'
															: 'Lancer le passage'}
													tone="tertiary"
													disabled={!job.machineId || isBusy(job.id)}
													onclick={() => startJob(job)}
												>
													{#if job.state === 'failed'}<RotateCcw size={15} />{:else}<Play
															size={15}
														/>{/if}
												</IconAction>
												<IconAction
													label="Supprimer le passage"
													tone="error"
													disabled={isBusy(job.id)}
													onclick={() => deleteJob(job)}><Trash2 size={15} /></IconAction
												>
											{:else if job.state === 'printing'}
												<IconAction
													label="Marquer le passage comme réussi"
													tone="success"
													disabled={isBusy(job.id)}
													onclick={() => completeJob(job)}><Check size={15} /></IconAction
												>
												<IconAction
													label="Signaler un échec"
													tone="error"
													onclick={() => beginFailure(job)}><TriangleAlert size={15} /></IconAction
												>
											{/if}
										</div>
									</td>
								</tr>
								{#if failureJobId === job.id}
									<tr class="bg-error-50-950">
										<td colspan="7" class="p-3">
											<div class="grid grid-cols-[minmax(0,1fr)_8rem_8rem_auto] items-end gap-3">
												<label class="label"
													>Motif<Input required bind:value={failureForm.reason} /></label
												>
												<label class="label"
													>Qté échouée<Input
														type="number"
														min="1"
														bind:value={failureForm.failedQuantity}
													/></label
												>
												<label class="label"
													>Déchet (g)<Input
														type="number"
														min="0"
														step="0.1"
														bind:value={failureForm.actualWasteGrams}
													/></label
												>
												<div class="flex gap-1">
													<IconAction
														label="Confirmer l’échec"
														tone="error"
														disabled={isBusy(job.id)}
														onclick={() => failJob(job)}><Check size={15} /></IconAction
													>
													<IconAction
														label="Annuler l’échec"
														tone="error"
														onclick={() => (failureJobId = null)}><X size={15} /></IconAction
													>
												</div>
											</div>
										</td>
									</tr>
								{/if}
							{/each}
							{#if addingJob}
								<tr class="bg-tertiary-50-950">
									<td class="p-2"
										><Input as="select" bind:value={newJobForm.pieceId}
											>{#each pieces as piece (piece.id)}<option value={piece.id}
													>{piece.label}</option
												>{/each}</Input
										></td
									>
									<td class="p-2"
										><Input type="number" min="1" bind:value={newJobForm.quantity} /></td
									>
									<td class="p-2"
										><Input as="select" bind:value={newJobForm.filamentSupplyId}
											>{#each filaments as filament (filament.id)}<option value={filament.id}
													>{filament.label}</option
												>{/each}</Input
										></td
									>
									<td class="p-2"
										><Input as="select" bind:value={newJobForm.machineId}
											><option value="">À affecter</option
											>{#each machines as machine (machine.id)}<option value={machine.id}
													>{machine.label}</option
												>{/each}</Input
										></td
									>
									<td class="px-3 py-2"><Badge variant="tonal" tone="surface">À imprimer</Badge></td
									>
									<td class="px-3 py-2">—</td>
									<td class="px-3 py-2">
										<div class="flex justify-end gap-1">
											<IconAction
												label="Ajouter le passage"
												tone="tertiary"
												disabled={isBusy('new-job')}
												onclick={addJob}><Check size={15} /></IconAction
											>
											<IconAction
												label="Annuler l’ajout"
												tone="error"
												onclick={() => (addingJob = false)}><X size={15} /></IconAction
											>
										</div>
									</td>
								</tr>
							{:else if task.state !== 'completed' && task.state !== 'cancelled'}
								<tr>
									<td colspan="7" class="!p-0">
										<button
											class="flex w-full items-center justify-center gap-2 px-3 py-3 text-sm font-medium text-surface-600-400 transition-colors hover:bg-tertiary-50-950 hover:text-tertiary-700-300 focus-visible:bg-tertiary-50-950 focus-visible:outline-2 focus-visible:outline-tertiary-500"
											type="button"
											onclick={beginAddJob}><Plus size={15} />Ajouter une pièce</button
										>
									</td>
								</tr>
							{/if}
						</tbody>
					</table>
				</div>
				{#if error}<p class="mt-3 mb-0 rounded-base preset-tonal-error p-3 text-sm" role="alert">
						{error}
					</p>{/if}
			</div>
		</td>
	</tr>
{/if}

<style>
	@media (max-width: 700px) {
		.job-table-wrap {
			overflow: visible;
			border: 0;
			background: transparent;
		}
		.job-table,
		.job-table tbody,
		.job-table tr {
			display: grid;
			width: 100%;
		}
		.job-table {
			min-width: 0;
		}
		.job-table thead {
			position: absolute;
			width: 1px;
			height: 1px;
			overflow: hidden;
			clip: rect(0 0 0 0);
			white-space: nowrap;
		}
		.job-table tbody {
			gap: 10px;
		}
		.job-table tbody tr {
			overflow: hidden;
			border: 1px solid var(--color-surface-300-700);
			border-radius: var(--radius-base);
			background: var(--color-surface-50-950);
		}
		.job-table td {
			display: grid;
			grid-template-columns: 6.5rem minmax(0, 1fr);
			align-items: center;
			gap: 10px;
			min-height: 44px;
			padding: 9px 12px;
			border-bottom: 1px solid var(--color-surface-200-800);
		}
		.job-table td::before {
			font-size: 0.72rem;
			font-weight: 700;
			letter-spacing: 0.04em;
			text-transform: uppercase;
			color: var(--color-surface-700-300);
		}
		.job-table td:nth-child(1)::before {
			content: 'Pièce';
		}
		.job-table td:nth-child(2)::before {
			content: 'Quantité';
		}
		.job-table td:nth-child(3)::before {
			content: 'Filament';
		}
		.job-table td:nth-child(4)::before {
			content: 'Machine';
		}
		.job-table td:nth-child(5)::before {
			content: 'État';
		}
		.job-table td:nth-child(6)::before {
			content: 'Début';
		}
		.job-table td:nth-child(7) {
			display: flex;
			justify-content: flex-end;
		}
		.job-table td:nth-child(7)::before,
		.job-table td[colspan]::before {
			display: none;
		}
		.job-table td[colspan] {
			display: block;
			border-bottom: 0;
		}
	}
</style>
