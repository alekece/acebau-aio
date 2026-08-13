<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import Cpu from '@lucide/svelte/icons/cpu';
	import Check from '@lucide/svelte/icons/check';
	import Copy from '@lucide/svelte/icons/copy';
	import Plus from '@lucide/svelte/icons/plus';
	import Printer from '@lucide/svelte/icons/printer';
	import Pencil from '@lucide/svelte/icons/pencil';
	import Trash2 from '@lucide/svelte/icons/trash-2';
	import X from '@lucide/svelte/icons/x';
	import Button from '$lib/components/ui/Button.svelte';
	import DataSourceNotice from '$lib/components/ui/DataSourceNotice.svelte';
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import IconAction from '$lib/components/ui/IconAction.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Modal from '$lib/components/ui/Modal.svelte';
	import MachineStatus from '$lib/components/machines/MachineStatus.svelte';
	import Kpi from '$lib/components/ui/Kpi.svelte';
	import ModuleHeader from '$lib/components/ui/ModuleHeader.svelte';
	import OnboardingPanel from '$lib/components/ui/OnboardingPanel.svelte';
	import PageShell from '$lib/components/ui/PageShell.svelte';
	import Table from '$lib/components/ui/Table.svelte';
	import TableSection from '$lib/components/ui/TableSection.svelte';
	import MetricInput from '$lib/components/forms/MetricInput.svelte';
	import RatioInput from '$lib/components/forms/RatioInput.svelte';
	import type { MetricDTO, PowerUnit, PriceUnit, RatioDTO, TimeUnit } from '$lib/unit';
	import type { PageProps } from './$types';

	type MachineModel = {
		id: string;
		brand: string;
		name: string;
		purchaseCost: MetricDTO<PriceUnit>;
		maintenanceCost: RatioDTO<PriceUnit, TimeUnit>;
		lifetime: MetricDTO<TimeUnit>;
		averagePower: MetricDTO<PowerUnit>;
	};

	type Machine = {
		id: string;
		surname: string;
		modelId: string;
		printingTime: MetricDTO<TimeUnit>;
		state: 'available' | 'running' | 'maintenance' | 'broken';
		model: MachineModel;
	};

	type ModelForm = {
		brand: string;
		name: string;
		purchaseCost: string;
		maintenanceCostValue: string;
		maintenanceCostUnit: string;
		lifetimeValue: string;
		lifetimeUnit: string;
		averagePowerValue: string;
		averagePowerUnit: string;
	};

	type MachineForm = {
		modelId: string;
		surname: string;
		state: Machine['state'];
	};

	let { data }: PageProps = $props();
	let models = $state<MachineModel[]>([]);
	let machines = $state<Machine[]>([]);
	let loading = $state(false);
	let saving = $state(false);
	let error = $state('');
	type ModalKind = 'models' | 'new-machine' | 'machine' | 'confirm-delete-machine' | null;
	let activeModal = $state<ModalKind>(null);
	let onboardingStep = $state<1 | 2 | 3>(1);
	let editingModelId = $state<string | null>(null);
	let selectedMachineId = $state<string | null>(null);
	let modelForm = $state<ModelForm>(emptyModel());
	let machineForm = $state<MachineForm>(emptyMachine());
	let machineEditForm = $state<{ surname: string; state: Machine['state'] }>({
		surname: '',
		state: 'available'
	});
	let machineEdits = $state<
		Record<string, { surname: string; state: Machine['state']; busy: boolean; error: string }>
	>({});

	$effect(() => {
		models = data.models;
		machines = data.machines;
		if (data.loadError) error = data.loadError;
	});
	let selectedMachine = $derived(machines.find((machine) => machine.id === selectedMachineId));
	let usableMachines = $derived(
		machines.filter((machine) => machine.state === 'available' || machine.state === 'running')
			.length
	);
	let unavailableMachines = $derived(
		machines.filter((machine) => machine.state === 'maintenance' || machine.state === 'broken')
			.length
	);
	const hardcodedPowerCost = '0,25 €/h';
	const hardcodedWorkload = '68 %';
	const hardcodedNextMaintenance = 'À planifier';
	const onboardingSteps = ['Créer un modèle', 'Ajouter une machine', 'Vérifier les informations'];
	const timeUnits = [
		{ value: 'min', label: 'minute' },
		{ value: 'h', label: 'heure' },
		{ value: 'd', label: 'jour' },
		{ value: 'mo', label: 'mois' },
		{ value: 'y', label: 'année' }
	];
	const maintenanceCostUnits = [
		{ value: 'min', label: 'minute' },
		{ value: 'h', label: 'heure' },
		{ value: 'd', label: 'jour' },
		{ value: 'mo', label: 'mois' },
		{ value: 'y', label: 'année' }
	];
	const priceUnits = [{ value: '€', label: '€' }];
	const powerUnits = [
		{ value: 'W', label: 'W' },
		{ value: 'kW', label: 'kW' }
	];

	function emptyModel(): ModelForm {
		return {
			brand: '',
			name: '',
			purchaseCost: '100',
			maintenanceCostValue: '5',
			maintenanceCostUnit: 'y',
			lifetimeValue: '10',
			lifetimeUnit: '',
			averagePowerValue: '100',
			averagePowerUnit: ''
		};
	}

	function metricLabel(metric: MetricDTO) {
		return `${metric.value}${metric.unit}`;
	}

	function ratioLabel(ratio: RatioDTO) {
		return `${ratio.value}${ratio.numeratorUnit}/${ratio.denominatorUnit}`;
	}

	function modelInput(form: ModelForm) {
		return {
			brand: form.brand,
			name: form.name,
			purchaseCost: { value: form.purchaseCost, unit: '€' },
			maintenanceCost: {
				value: form.maintenanceCostValue,
				numeratorUnit: '€',
				denominatorUnit: form.maintenanceCostUnit
			},
			lifetime: { value: form.lifetimeValue, unit: form.lifetimeUnit },
			averagePower: { value: form.averagePowerValue, unit: form.averagePowerUnit }
		};
	}

	function emptyMachine(): MachineForm {
		return { modelId: '', surname: '', state: 'available' };
	}

	async function gql<T>(query: string, variables: Record<string, unknown> = {}): Promise<T> {
		const response = await fetch('/api/graphql', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ query, variables })
		});
		const result = await response.json();
		if (!response.ok || result.errors?.length) {
			const message =
				result.errors?.map((item: { message: string }) => item.message).join('; ') ??
				response.statusText;
			throw new Error(message || 'La requête GraphQL a échoué.');
		}
		return result.data;
	}

	function openModelManager() {
		editingModelId = null;
		modelForm = emptyModel();
		activeModal = 'models';
	}

	function editModel(model: MachineModel) {
		editingModelId = model.id;
		modelForm = {
			brand: model.brand,
			name: model.name,
			purchaseCost: model.purchaseCost.value,
			maintenanceCostValue: model.maintenanceCost.value,
			maintenanceCostUnit: model.maintenanceCost.denominatorUnit,
			lifetimeValue: model.lifetime.value,
			lifetimeUnit: model.lifetime.unit,
			averagePowerValue: model.averagePower.value,
			averagePowerUnit: model.averagePower.unit
		};
	}

	function copyModel(model: MachineModel) {
		editModel(model);
		editingModelId = null;
		modelForm.name = `${model.name} copie`;
	}

	function addModel() {
		editingModelId = null;
		modelForm = emptyModel();
	}

	function openMachineForm() {
		machineForm = { ...emptyMachine(), modelId: models[0]?.id ?? '' };
		activeModal = 'new-machine';
	}

	function openMachineDetails(machine: Machine) {
		selectedMachineId = machine.id;
		machineEditForm = { surname: machine.surname, state: machine.state };
		activeModal = 'machine';
	}

	function requestDeleteMachine(machine: Machine) {
		selectedMachineId = machine.id;
		activeModal = 'confirm-delete-machine';
	}

	function closeModal() {
		activeModal = null;
		onboardingStep = 1;
		editingModelId = null;
		selectedMachineId = null;
	}

	function handleModalChange(details: { open: boolean }) {
		if (!details.open) closeModal();
	}

	function machineStateLabel(state: Machine['state']): string {
		return {
			available: 'Disponible',
			running: 'En production',
			maintenance: 'Maintenance',
			broken: 'En panne'
		}[state];
	}

	async function saveModel() {
		saving = true;
		error = '';
		try {
			const input = modelInput(modelForm);
			if (editingModelId) {
				await gql(
					`mutation($id: String!, $input: MachineModelInput!) { updateMachineModel(id: $id, input: $input) { id } }`,
					{ id: editingModelId, input }
				);
			} else {
				await gql(
					`mutation($input: MachineModelInput!) { createMachineModel(input: $input) { id } }`,
					{ input }
				);
			}
			if (activeModal !== 'models') closeModal();
			else addModel();
			await invalidateAll();
		} catch (cause) {
			console.error('Impossible de créer le modèle.', cause);
			error = `Le modèle ne peut pas être créé. ${cause instanceof Error ? cause.message : 'Vérifiez les valeurs saisies.'}`;
		} finally {
			saving = false;
		}
	}

	async function saveOnboarding() {
		saving = true;
		error = '';
		try {
			const result = await gql<{ createMachineModel: { id: string } }>(
				`mutation($input: MachineModelInput!) { createMachineModel(input: $input) { id } }`,
				{ input: modelInput(modelForm) }
			);
			await gql(`mutation($input: MachineInput!) { createMachine(input: $input) { id } }`, {
				input: {
					...machineForm,
					modelId: result.createMachineModel.id,
					printingTime: { value: '0', unit: 'h' }
				}
			});
			closeModal();
			await invalidateAll();
		} catch (cause) {
			error = `L’atelier ne peut pas être initialisé. ${cause instanceof Error ? cause.message : 'Vérifiez les valeurs saisies.'}`;
		} finally {
			saving = false;
		}
	}

	async function saveMachine() {
		saving = true;
		error = '';
		try {
			await gql(
				`mutation($input: MachineInput!) { createMachine(input: $input) { id } }`,
				{ input: { ...machineForm, printingTime: { value: '0', unit: 'h' } } }
			);
			closeModal();
			await invalidateAll();
		} catch (cause) {
			console.error('Impossible de créer la machine.', cause);
			error = `La machine ne peut pas être créée. ${cause instanceof Error ? cause.message : 'Vérifiez le modèle sélectionné.'}`;
		} finally {
			saving = false;
		}
	}

	async function deleteModel(id: string) {
		const machineCount = machines.filter((machine) => machine.modelId === id).length;
		if (machineCount > 0) {
			error = `Ce modèle ne peut pas être supprimé : ${machineCount} machine${machineCount > 1 ? 's lui sont' : ' lui est'} encore associée${machineCount > 1 ? 's' : ''}.`;
			return;
		}
		saving = true;
		error = '';
		try {
			await gql(`mutation($id: String!) { deleteMachineModel(id: $id) }`, { id });
			if (editingModelId === id) addModel();
			await invalidateAll();
		} catch (cause) {
			error = `Le modèle ne peut pas être supprimé. ${cause instanceof Error ? cause.message : ''}`;
		} finally {
			saving = false;
		}
	}

	async function saveMachineDetails() {
		if (!selectedMachineId) return;
		saving = true;
		error = '';
		try {
			await gql(
				`mutation($id: String!, $input: MachineChangeset!) { patchMachine(id: $id, input: $input) { id } }`,
				{ id: selectedMachineId, input: machineEditForm }
			);
			closeModal();
			await invalidateAll();
		} catch (cause) {
			error = `La machine ne peut pas être modifiée. ${cause instanceof Error ? cause.message : ''}`;
		} finally {
			saving = false;
		}
	}

	async function deleteMachine() {
		if (!selectedMachineId) return;
		const deletedMachineId = selectedMachineId;
		saving = true;
		error = '';
		try {
			await gql(`mutation($id: String!) { deleteMachine(id: $id) }`, { id: deletedMachineId });
			machines = machines.filter((machine) => machine.id !== deletedMachineId);
			closeModal();
			await invalidateAll();
		} catch (cause) {
			error = `La machine ne peut pas être supprimée. ${cause instanceof Error ? cause.message : ''}`;
		} finally {
			saving = false;
		}
	}

	function beginMachineEdit(machine: Machine) {
		machineEdits = {
			...machineEdits,
			[machine.id]: { surname: machine.surname, state: machine.state, busy: false, error: '' }
		};
	}

	function cancelMachineEdit(id: string) {
		const remaining = { ...machineEdits };
		delete remaining[id];
		machineEdits = remaining;
	}

	async function saveInlineMachine(id: string) {
		const edit = machineEdits[id];
		if (!edit?.surname.trim()) return;
		machineEdits = { ...machineEdits, [id]: { ...edit, busy: true, error: '' } };
		try {
			await gql(
				`mutation($id: String!, $input: MachineChangeset!) { patchMachine(id: $id, input: $input) { id } }`,
				{ id, input: { surname: edit.surname.trim(), state: edit.state } }
			);
			cancelMachineEdit(id);
			await invalidateAll();
		} catch (cause) {
			machineEdits = {
				...machineEdits,
				[id]: {
					...edit,
					busy: false,
					error: cause instanceof Error ? cause.message : 'Impossible de modifier la machine.'
				}
			};
		}
	}
</script>

<svelte:head><title>Machines — Acebau</title></svelte:head>

<PageShell>
	<ModuleHeader title="Machines">
		{#snippet actions()}
			{#if models.length > 0 && machines.length > 0}
				<Button variant="outlined" tone="secondary" onclick={openModelManager}>
					<Plus size={17} />Nouveau modèle
				</Button>
				<Button tone="tertiary" onclick={openMachineForm} disabled={models.length === 0}>
					<Plus size={17} />Nouvelle machine
				</Button>
			{/if}
		{/snippet}
	</ModuleHeader>
	<DataSourceNotice visible={data.usingFallback} message={data.loadError} />

	{#if error}<p class="mb-5 rounded-base preset-tonal-error p-3" role="alert">{error}</p>{/if}

	{#if loading}
		<p class="sr-only" aria-live="polite">Chargement des machines…</p>
		<section
			class="mb-6 grid grid-cols-4 gap-3.5 max-[1000px]:grid-cols-2 max-[850px]:grid-cols-1"
			aria-label="Chargement des indicateurs"
		>
			<Kpi loading />
			<Kpi loading />
			<Kpi loading />
			<Kpi loading />
		</section>
		<section
			class="card border border-surface-300-700 bg-surface-50-950 p-7 shadow-sm max-[850px]:p-4"
			aria-label="Chargement du parc machines"
		>
			<div class="mb-5 grid gap-2">
				<span class="h-5 placeholder w-40 animate-pulse"></span>
				<span class="h-3 placeholder w-64 animate-pulse"></span>
			</div>
			<Table loading loadingColumns={9} />
		</section>
	{:else if models.length === 0 || machines.length === 0}
		{#if models.length === 0}
			<OnboardingPanel
				title="Configurons votre parc"
				steps={onboardingSteps}
				currentStep={onboardingStep - 1}
			>
				{#if onboardingStep === 1}
					<form
						onsubmit={(event) => {
							event.preventDefault();
							onboardingStep = 2;
						}}
						class="grid gap-5"
					>
						<div>
							<h3 class="mb-1 text-lg font-semibold text-surface-900-100">Décrivez votre modèle</h3>
							<p class="m-0 text-sm text-surface-700-300">
								Ces informations seront réutilisées pour toutes les machines de ce type.
							</p>
						</div>
						<div class="grid grid-cols-2 gap-3 max-[600px]:grid-cols-1">
							<label class="label"
								>Marque<Input
									required
									bind:value={modelForm.brand}
									placeholder="Ex. Prusa"
								/></label
							>
							<label class="label"
								>Nom<Input required bind:value={modelForm.name} placeholder="Ex. MK4" /></label
							>
							<MetricInput
								label="Prix d’achat"
								required
								bind:value={modelForm.purchaseCost}
								unit="€"
								units={priceUnits}
							/>
							<RatioInput
								label="Coût de maintenance"
								required
								bind:value={modelForm.maintenanceCostValue}
								numeratorUnit="€"
								bind:denominatorUnit={modelForm.maintenanceCostUnit}
								numeratorUnits={priceUnits}
								denominatorUnits={maintenanceCostUnits}
							/>
							<MetricInput
								label="Durée de vie"
								required
								kind="time"
								bind:value={modelForm.lifetimeValue}
								bind:unit={modelForm.lifetimeUnit}
								units={timeUnits}
							/>
							<MetricInput
								label="Puissance moyenne"
								required
								kind="power"
								bind:value={modelForm.averagePowerValue}
								bind:unit={modelForm.averagePowerUnit}
								units={powerUnits}
							/>
						</div>
						<div class="flex justify-end border-t border-surface-300-700 pt-5">
							<Button type="submit" tone="tertiary">Continuer<Plus size={16} /></Button>
						</div>
					</form>
				{:else if onboardingStep === 2}
					<form
						onsubmit={(event) => {
							event.preventDefault();
							onboardingStep = 3;
						}}
						class="grid gap-5"
					>
						<div>
							<h3 class="mb-1 text-lg font-semibold text-surface-900-100">
								Ajoutez votre première machine
							</h3>
							<p class="m-0 text-sm text-surface-700-300">
								Donnez-lui un nom pour la retrouver facilement dans votre parc.
							</p>
						</div>
						<div class="grid grid-cols-2 gap-3 max-[600px]:grid-cols-1">
							<label class="label"
								>Surnom<Input
									required
									bind:value={machineForm.surname}
									placeholder="Ex. K2 du fond"
								/></label
							><label class="label"
								>État<Input as="select" bind:value={machineForm.state}
									><option value="available">Disponible</option><option value="running"
										>En production</option
									><option value="maintenance">Maintenance</option><option value="broken"
										>En panne</option
									></Input
								></label
							>
						</div>
						<div class="flex justify-between gap-2.5">
							<Button
								variant="outlined"
								tone="secondary"
								type="button"
								onclick={() => (onboardingStep = 1)}>Retour</Button
							><Button type="submit" tone="tertiary">Continuer</Button>
						</div>
					</form>
				{:else}
					<div class="grid gap-5">
						<div>
							<h3 class="mb-1 text-lg font-semibold text-surface-900-100">
								Vérifiez votre configuration
							</h3>
							<p class="m-0 text-sm text-surface-700-300">
								Tout est prêt. Vérifiez ces informations avant de créer votre atelier.
							</p>
						</div>
						<div class="grid gap-3 sm:grid-cols-2">
							<div class="rounded-base border border-surface-300-700 bg-surface-100-900 p-4">
								<p class="mb-2 text-xs font-bold tracking-wider text-surface-700-300 uppercase">
									Modèle
								</p>
								<strong class="block text-base text-surface-900-100"
									>{modelForm.brand} · {modelForm.name}</strong
								><small class="text-surface-700-300"
									>{modelForm.averagePowerValue}{modelForm.averagePowerUnit} · durée de vie {modelForm.lifetimeValue}{modelForm.lifetimeUnit}</small
								>
							</div>
							<div class="rounded-base border border-surface-300-700 bg-surface-100-900 p-4">
								<p class="mb-2 text-xs font-bold tracking-wider text-surface-700-300 uppercase">
									Première machine
								</p>
								<strong class="block text-base text-surface-900-100">{machineForm.surname}</strong
								><small class="text-surface-700-300"
									>{machineStateLabel(machineForm.state)} · modèle assigné</small
								>
							</div>
						</div>
						<div class="flex justify-between gap-2.5 border-t border-surface-300-700 pt-5">
							<Button
								variant="outlined"
								tone="secondary"
								type="button"
								onclick={() => (onboardingStep = 2)}>Retour</Button
							><Button type="button" tone="tertiary" disabled={saving} onclick={saveOnboarding}
								>{saving ? 'Création…' : 'Créer mon atelier'}</Button
							>
						</div>
					</div>
				{/if}
			</OnboardingPanel>
		{:else}
			<OnboardingPanel title="Configurons votre parc" steps={onboardingSteps} currentStep={1}>
				<EmptyState
					compact
					title="Ajoutez votre première machine"
					description="Le modèle est prêt. Ajoutez maintenant la machine physique que vous utiliserez dans l’atelier."
				>
					{#snippet icon()}<Printer size={25} />{/snippet}
					{#snippet actions()}<Button tone="tertiary" onclick={openMachineForm}
							><Plus size={17} />Ajouter une machine</Button
						>{/snippet}
				</EmptyState>
			</OnboardingPanel>
		{/if}
	{:else}
		<section
			class="mb-6 grid grid-cols-4 gap-3.5 max-[1000px]:grid-cols-2 max-[850px]:grid-cols-1"
			aria-label="Indicateurs machines"
		>
			<Kpi
				label="Machines utilisables"
				value={`${usableMachines} / ${machines.length}`}
				detail="Disponibles ou en production"
			/>
			<Kpi label="Charge à 7 jours" value="—" />
			<Kpi label="Échecs / 100 h · 30 jours" value="—" />
			<Kpi label="Maintenance à prévoir" value={unavailableMachines} />
		</section>
		<TableSection title="Parc">
			{#snippet toolbar()}
				<button
					class="badge cursor-pointer preset-tonal-surface transition [--badge-size:var(--text-sm)] hover:brightness-95"
					type="button"
					onclick={openModelManager}
					><Cpu size={16} />{models.length} modèle{models.length > 1 ? 's' : ''}</button
				>
			{/snippet}
			<Table responsiveCards>
				<thead
					><tr
						><th>#</th><th>Machine</th><th>Modèle</th><th>État</th><th>Charge</th><th
							>Temps d’impression</th
						><th>Prochaine maintenance</th><th>Coût effectif</th><th
							><span class="sr-only">Actions</span></th
						></tr
					></thead
				>
				<tbody>
					{#each machines as machine, index (machine.id)}
						<tr
							class="cursor-pointer transition hover:bg-surface-100-900"
							onclick={() => openMachineDetails(machine)}
							onkeydown={(event) => event.key === 'Enter' && openMachineDetails(machine)}
							tabindex="0"
						>
							<td data-label="" class="mobile-card-hidden text-surface-700-300">{index + 1}</td>
							<td data-label="Machine" onclick={(event) => event.stopPropagation()}>
								{#if machineEdits[machine.id]}
									<div class="grid gap-2">
										<Input
											bind:value={machineEdits[machine.id].surname}
											aria-label="Surnom de la machine"
										/>
										{#if machineEdits[machine.id].error}<small
												class="text-error-600-400"
												role="alert">{machineEdits[machine.id].error}</small
											>{/if}
									</div>
								{:else}<strong>{machine.surname}</strong><small>#{machine.id.slice(0, 8)}</small
									>{/if}
							</td>
							<td data-label="Modèle"
								><strong>{machine.model?.name ?? 'Modèle indisponible'}</strong><small
									>{machine.model?.brand ?? '—'}</small
								></td
							>
							<td data-label="État" onclick={(event) => event.stopPropagation()}
								>{#if machineEdits[machine.id]}<Input
										as="select"
										bind:value={machineEdits[machine.id].state}
										><option value="available">Disponible</option><option value="running"
											>En production</option
										><option value="maintenance">Maintenance</option><option value="broken"
											>En panne</option
										></Input
									>{:else}<MachineStatus state={machine.state} />{/if}</td
							>
							<td data-label="Charge">{hardcodedWorkload}</td>
							<td data-label="Temps d’impression">{metricLabel(machine.printingTime)}</td>
							<td data-label="Prochaine maintenance">{hardcodedNextMaintenance}</td>
							<td data-label="Coût effectif">{hardcodedPowerCost}</td>
							<td
								data-label=""
								class="mobile-card-actions"
								onclick={(event) => event.stopPropagation()}
							>
								{#if machineEdits[machine.id]}
									<IconAction
										label={`Confirmer la modification de ${machine.surname}`}
										tone="success"
										disabled={machineEdits[machine.id].busy}
										onclick={() => saveInlineMachine(machine.id)}><Check size={18} /></IconAction
									>
									<IconAction
										label={`Annuler la modification de ${machine.surname}`}
										tone="error"
										disabled={machineEdits[machine.id].busy}
										onclick={() => cancelMachineEdit(machine.id)}><X size={18} /></IconAction
									>
								{:else}
									<IconAction
										label={`Modifier ${machine.surname}`}
										onclick={() => beginMachineEdit(machine)}><Pencil size={18} /></IconAction
									>
									<IconAction
										label={`Supprimer ${machine.surname}`}
										tone="error"
										onclick={() => requestDeleteMachine(machine)}><Trash2 size={18} /></IconAction
									>
								{/if}
							</td>
						</tr>
					{/each}
				</tbody>
			</Table>
		</TableSection>
	{/if}

	{#if activeModal === 'models'}
		<Modal
			open={true}
			onOpenChange={handleModalChange}
			contentClasses="w-full max-w-2xl rounded-container border border-surface-300-700 bg-surface-50-950 p-7 shadow-xl max-[600px]:p-5"
		>
			{#snippet content()}
				<div class="mb-6 flex items-start justify-between gap-4">
					<div>
						<p class="mb-1 text-xs font-semibold tracking-wider text-surface-700-300 uppercase">
							Référentiel
						</p>
						<h2 class="text-2xl font-semibold text-surface-900-100">Modèles de machines</h2>
					</div>
					<button
						class="btn-icon preset-tonal-surface"
						type="button"
						aria-label="Fermer"
						onclick={closeModal}><X size={18} /></button
					>
				</div>
				<div class="mb-5 grid gap-2">
					{#each models as model (model.id)}{@const machineCount = machines.filter(
							(machine) => machine.modelId === model.id
						).length}
						<div
							class="flex items-center justify-between gap-3 rounded-base border border-surface-300-700 p-3"
						>
							<div>
								<strong class="block text-sm text-surface-900-100"
									>{model.brand} · {model.name}</strong
								><small class="text-surface-700-300"
									>{metricLabel(model.averagePower)} · durée de vie {metricLabel(model.lifetime)}{#if machineCount > 0}
										· {machineCount} machine{machineCount > 1 ? 's' : ''}{/if}</small
								>
							</div>
							<div class="flex gap-1">
								<IconAction label={`Modifier ${model.name}`} onclick={() => editModel(model)}
									><Pencil size={16} /></IconAction
								>
								<IconAction label={`Copier ${model.name}`} onclick={() => copyModel(model)}
									><Copy size={16} /></IconAction
								>
								<IconAction
									label={machineCount > 0
										? `Impossible de supprimer ${model.name}, modèle utilisé`
										: `Supprimer ${model.name}`}
									tone="error"
									disabled={saving || machineCount > 0}
									onclick={() => deleteModel(model.id)}><Trash2 size={16} /></IconAction
								>
							</div>
						</div>{/each}
				</div>
				<form
					onsubmit={(event) => {
						event.preventDefault();
						saveModel();
					}}
					class="grid gap-4 border-t border-surface-300-700 pt-5"
				>
					<h3 class="text-lg font-semibold text-surface-900-100">
						{editingModelId ? 'Modifier le modèle' : 'Ajouter un modèle'}
					</h3>
					<div class="grid grid-cols-2 gap-3 max-[600px]:grid-cols-1">
						<label class="label">Marque<Input required bind:value={modelForm.brand} /></label><label
							class="label">Nom<Input required bind:value={modelForm.name} /></label
						><MetricInput
							label="Prix d’achat"
							required
							bind:value={modelForm.purchaseCost}
							unit="€"
							units={priceUnits}
						/><RatioInput
							label="Coût de maintenance"
							required
							bind:value={modelForm.maintenanceCostValue}
							numeratorUnit="€"
							bind:denominatorUnit={modelForm.maintenanceCostUnit}
							numeratorUnits={priceUnits}
							denominatorUnits={maintenanceCostUnits}
						/><MetricInput
							label="Durée de vie"
							required
							kind="time"
							bind:value={modelForm.lifetimeValue}
							bind:unit={modelForm.lifetimeUnit}
							units={timeUnits}
						/><MetricInput
							label="Puissance moyenne"
							required
							kind="power"
							bind:value={modelForm.averagePowerValue}
							bind:unit={modelForm.averagePowerUnit}
							units={powerUnits}
						/>
					</div>
					<div class="flex justify-end gap-2.5">
						<Button variant="outlined" tone="secondary" type="button" onclick={addModel}
							>Nouveau</Button
						><Button type="submit" tone="tertiary" disabled={saving}
							>{saving ? 'Enregistrement…' : editingModelId ? 'Enregistrer' : 'Ajouter'}</Button
						>
					</div>
				</form>
			{/snippet}
		</Modal>
	{/if}

	{#if activeModal === 'new-machine'}
		<Modal
			open={true}
			onOpenChange={handleModalChange}
			contentClasses="w-full max-w-xl rounded-container border border-surface-300-700 bg-surface-50-950 p-7 shadow-xl max-[600px]:p-5"
		>
			{#snippet content()}
				<div class="mb-6 flex items-start justify-between gap-4">
					<div>
						<p class="mb-1 text-xs font-semibold tracking-wider text-surface-700-300 uppercase">
							Parc physique
						</p>
						<h2 class="text-2xl font-semibold text-surface-900-100">Nouvelle machine</h2>
					</div>
					<button
						class="btn-icon preset-tonal-surface"
						type="button"
						aria-label="Fermer"
						onclick={closeModal}><X size={18} /></button
					>
				</div>
				<form
					onsubmit={(event) => {
						event.preventDefault();
						saveMachine();
					}}
					class="grid gap-4"
				>
					<div class="grid grid-cols-2 gap-3 max-[600px]:grid-cols-1">
						<label class="label"
							>Modèle<Input as="select" required bind:value={machineForm.modelId}
								><option value="" disabled>Choisir un modèle</option
								>{#each models as model (model.id)}<option value={model.id}
										>{model.brand} · {model.name}</option
									>{/each}</Input
							></label
						><label class="label"
							>Surnom<Input
								required
								bind:value={machineForm.surname}
								placeholder="Ex. K2 du fond"
							/></label
						><label class="label"
							>État<Input as="select" bind:value={machineForm.state}
								><option value="available">Disponible</option><option value="running"
									>En production</option
								><option value="maintenance">Maintenance</option><option value="broken"
									>En panne</option
								></Input
							></label
						>
					</div>
					<div class="flex justify-end gap-2.5">
						<Button variant="outlined" tone="secondary" onclick={closeModal}>Annuler</Button><Button
							type="submit"
							tone="tertiary"
							disabled={saving}>{saving ? 'Création…' : 'Créer la machine'}</Button
						>
					</div>
				</form>
			{/snippet}
		</Modal>
	{/if}

	{#if activeModal === 'machine' && selectedMachine}
		<Modal
			open={true}
			onOpenChange={handleModalChange}
			contentClasses="w-full max-w-xl rounded-container border border-surface-300-700 bg-surface-50-950 p-7 shadow-xl max-[600px]:p-5"
		>
			{#snippet content()}
				<div class="mb-6 flex items-start justify-between gap-4">
					<div>
						<p class="mb-1 text-xs font-semibold tracking-wider text-surface-700-300 uppercase">
							Détail machine
						</p>
						<h2 class="text-2xl font-semibold text-surface-900-100">{selectedMachine.surname}</h2>
					</div>
					<button
						class="btn-icon preset-tonal-surface"
						type="button"
						aria-label="Fermer"
						onclick={closeModal}><X size={18} /></button
					>
				</div>
				<div class="mb-5 grid grid-cols-2 gap-3 rounded-base bg-surface-100-900 p-4 text-sm">
					<div>
						<span class="block text-xs text-surface-700-300">Modèle</span><strong
							>{selectedMachine.model?.brand} · {selectedMachine.model?.name}</strong
						>
					</div>
					<div>
						<span class="block text-xs text-surface-700-300">Temps d’impression</span><strong
							>{metricLabel(selectedMachine.printingTime)}</strong
						>
					</div>
					<div>
						<span class="block text-xs text-surface-700-300">Charge</span><strong
							>{hardcodedWorkload}</strong
						>
					</div>
					<div>
						<span class="block text-xs text-surface-700-300">Prochaine maintenance</span><strong
							>{hardcodedNextMaintenance}</strong
						>
					</div>
					<div>
						<span class="block text-xs text-surface-700-300">Coût effectif</span><strong
							>{hardcodedPowerCost}</strong
						>
					</div>
					<div>
						<span class="block text-xs text-surface-700-300">Puissance moyenne</span><strong
							>{selectedMachine.model ? metricLabel(selectedMachine.model.averagePower) : '—'}</strong
						>
					</div>
					<div>
						<span class="block text-xs text-surface-700-300">Prix d’achat</span><strong
							>{selectedMachine.model ? metricLabel(selectedMachine.model.purchaseCost) : '—'}</strong
						>
					</div>
					<div>
						<span class="block text-xs text-surface-700-300">Maintenance</span><strong
							>{selectedMachine.model ? ratioLabel(selectedMachine.model.maintenanceCost) : '—'}</strong
						>
					</div>
					<div>
						<span class="block text-xs text-surface-700-300">Durée de vie</span><strong
							>{selectedMachine.model ? metricLabel(selectedMachine.model.lifetime) : '—'}</strong
						>
					</div>
				</div>
				<form
					onsubmit={(event) => {
						event.preventDefault();
						saveMachineDetails();
					}}
					class="grid gap-4"
				>
					<label class="label">Surnom<Input required bind:value={machineEditForm.surname} /></label
					><label class="label"
						>État<Input as="select" bind:value={machineEditForm.state}
							><option value="available">Disponible</option><option value="running"
								>En production</option
							><option value="maintenance">Maintenance</option><option value="broken"
								>En panne</option
							></Input
						></label
					>
					<div class="flex justify-between gap-2.5">
						<Button
							variant="outlined"
							tone="error"
							type="button"
							onclick={deleteMachine}
							disabled={saving}><Trash2 size={16} />Supprimer</Button
						>
						<div class="flex gap-2.5">
							<Button variant="outlined" tone="secondary" onclick={closeModal}>Annuler</Button
							><Button type="submit" tone="tertiary" disabled={saving}
								>{saving ? 'Enregistrement…' : 'Enregistrer'}</Button
							>
						</div>
					</div>
				</form>
			{/snippet}
		</Modal>
	{/if}

	{#if activeModal === 'confirm-delete-machine' && selectedMachine}
		<Modal
			open={true}
			onOpenChange={handleModalChange}
			contentClasses="w-full max-w-md rounded-container border border-surface-300-700 bg-surface-50-950 p-7 shadow-xl max-[600px]:p-5"
		>
			{#snippet content()}
				<div class="mb-6 flex items-start justify-between gap-4">
					<div>
						<p class="mb-1 text-xs font-semibold tracking-wider text-surface-700-300 uppercase">
							Supprimer la machine
						</p>
						<h2 class="text-2xl font-semibold text-surface-900-100">Confirmer la suppression</h2>
					</div>
					<button
						class="btn-icon preset-tonal-surface"
						type="button"
						aria-label="Fermer"
						onclick={closeModal}><X size={18} /></button
					>
				</div>
				<p class="mb-6 text-sm text-surface-700-300">
					La machine <strong class="text-surface-900-100">{selectedMachine.surname}</strong> sera définitivement
					supprimée du parc. Cette action est irréversible.
				</p>
				<div class="flex justify-end gap-2.5">
					<Button variant="outlined" tone="secondary" type="button" onclick={closeModal}
						>Annuler</Button
					><Button tone="error" type="button" onclick={deleteMachine} disabled={saving}
						>{saving ? 'Suppression…' : 'Supprimer'}</Button
					>
				</div>
			{/snippet}
		</Modal>
	{/if}
</PageShell>
