<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { tick } from 'svelte';
	import Cpu from '@lucide/svelte/icons/cpu';
	import Check from '@lucide/svelte/icons/check';
	import Copy from '@lucide/svelte/icons/copy';
	import Plus from '@lucide/svelte/icons/plus';
	import Printer from '@lucide/svelte/icons/printer';
	import Pencil from '@lucide/svelte/icons/pencil';
	import Trash2 from '@lucide/svelte/icons/trash-2';
	import X from '@lucide/svelte/icons/x';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import DataSourceNotice from '$lib/components/ui/DataSourceNotice.svelte';
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import IconAction from '$lib/components/ui/IconAction.svelte';
	import Input from '$lib/components/ui/forms/Input.svelte';
	import Modal from '$lib/components/ui/Modal.svelte';
	import MachineStatus from '$lib/components/machines/MachineStatus.svelte';
	import SelectableBadge from '$lib/components/ui/SelectableBadge.svelte';
	import Kpi from '$lib/components/ui/Kpi.svelte';
	import ModuleHeader from '$lib/components/ui/ModuleHeader.svelte';
	import OnboardingPanel from '$lib/components/ui/OnboardingPanel.svelte';
	import PageShell from '$lib/components/ui/PageShell.svelte';
	import Table from '$lib/components/ui/Table.svelte';
	import TableDraftRow from '$lib/components/ui/TableDraftRow.svelte';
	import TableSection from '$lib/components/ui/TableSection.svelte';
	import MetricInput from '$lib/components/ui/forms/MetricInput.svelte';
	import RatioInput from '$lib/components/ui/forms/RatioInput.svelte';
	import { RequiredFeedback } from '$lib/components/ui/presets';
	import type { MetricDTO, PowerUnit, PriceUnit, RatioDTO, TimeUnit } from '$lib/unit';
	import type { PageProps } from './$types';

	type MachineState = 'available' | 'running' | 'maintenance' | 'broken';

	const machineStateOptions = [
		{ value: 'available', label: 'Disponible', tone: 'success' },
		{ value: 'running', label: 'En production', tone: 'secondary' },
		{ value: 'maintenance', label: 'Maintenance', tone: 'warning' },
		{ value: 'broken', label: 'En panne', tone: 'error' }
	] satisfies ReadonlyArray<{ value: MachineState; label: string; tone: string }>;

	type MachineModel = {
		id: string;
		brand: string;
		name: string;
		maintenanceCost: RatioDTO<PriceUnit, TimeUnit>;
		lifetime: MetricDTO<TimeUnit>;
		averagePower: MetricDTO<PowerUnit>;
	};

	type Machine = {
		id: string;
		surname: string;
		modelId: string;
		purchaseCost: MetricDTO<PriceUnit>;
		printingTime: MetricDTO<TimeUnit>;
		state: MachineState;
		model: MachineModel;
	};
	type MachinePage = {
		items: Machine[];
		page: number;
		pageSize: number;
		totalItems: number;
		totalPages: number;
	};

	type ModelForm = {
		brand: string;
		name: string;
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
		purchaseCost: string;
	};
	type MachineDraft = MachineForm & {
		id: string;
		busy: boolean;
		error: string;
		validationAttempt: number;
		animateRemoval: boolean;
	};

	let { data }: PageProps = $props();
	let models = $state<MachineModel[]>([]);
	let machines = $state<Machine[]>([]);
	let machinePage = $state<Omit<MachinePage, 'items'>>({
		page: 1,
		pageSize: 10,
		totalItems: 0,
		totalPages: 0
	});
	let pageLoading = $state(false);
	let loading = $state(false);
	let saving = $state(false);
	let error = $state('');
	type ModalKind = 'models' | 'new-machine' | 'confirm-delete-machine' | null;
	let activeModal = $state<ModalKind>(null);
	let onboardingStep = $state<1 | 2 | 3>(1);
	let editingModelId = $state<string | null>(null);
	let selectedMachineId = $state<string | null>(null);
	let modelForm = $state<ModelForm>(emptyModel());
	let machineForm = $state<MachineForm>(emptyMachine());
	let machineEdits = $state<
		Record<
			string,
			{
				modelId: string;
				surname: string;
				state: Machine['state'];
				busy: boolean;
				error: string;
			}
		>
	>({});
	let machineDrafts = $state<MachineDraft[]>([]);
	let pinnedMachineIds = $state<string[]>([]);
	let hasUnsavedTableRows = $derived(
		machineDrafts.length > 0 || Object.keys(machineEdits).length > 0
	);

	$effect(() => {
		models = data.models;
		machines = data.machines;
		machinePage = {
			page: data.machinePage.page,
			pageSize: data.machinePage.pageSize,
			totalItems: data.machinePage.totalItems,
			totalPages: data.machinePage.totalPages
		};
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
		return { modelId: '', surname: '', purchaseCost: '' };
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

	async function loadMachinePage(page: number) {
		if (page < 1 || pageLoading) return;
		if (
			hasUnsavedTableRows &&
			!window.confirm('Des modifications sont en cours. Changer de page entraînera leur perte.')
		)
			return;

		pageLoading = true;
		error = '';
		try {
			machineDrafts = [];
			machineEdits = {};
			pinnedMachineIds = [];
			const result = await gql<{ machines: MachinePage }>(
				`query MachineTablePage($page: Int!, $pageSize: Int!) {
					machines(page: $page, pageSize: $pageSize) {
						items {
							id surname modelId purchaseCost { value unit } printingTime { value unit } state
							model {
								id brand name
								maintenanceCost { value numeratorUnit denominatorUnit }
								lifetime { value unit }
								averagePower { value unit }
							}
						}
						page pageSize totalItems totalPages
					}
				}`,
				{ page, pageSize: 10 }
			);
			machines = result.machines.items;
			machinePage = {
				page: result.machines.page,
				pageSize: result.machines.pageSize,
				totalItems: result.machines.totalItems,
				totalPages: result.machines.totalPages
			};
		} catch (cause) {
			error = `La page des machines ne peut pas être chargée. ${cause instanceof Error ? cause.message : ''}`;
		} finally {
			pageLoading = false;
		}
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

	function addMachineDraft() {
		machineDrafts = [
			{
				...emptyMachine(),
				id: crypto.randomUUID(),
				modelId: models[0]?.id ?? '',
				busy: false,
				error: '',
				validationAttempt: 0,
				animateRemoval: true
			},
			...machineDrafts
		];
	}

	function cancelMachineDraft(id: string) {
		machineDrafts = machineDrafts.filter((draft) => draft.id !== id);
	}

	function updateMachineDraft(id: string, changes: Partial<MachineDraft>) {
		machineDrafts = machineDrafts.map((draft) =>
			draft.id === id ? { ...draft, ...changes } : draft
		);
	}

	async function saveMachineDraft(id: string) {
		const draft = machineDrafts.find((candidate) => candidate.id === id);
		if (!draft || draft.busy) return;
		updateMachineDraft(id, { validationAttempt: draft.validationAttempt + 1 });
		if (!draft.modelId || !draft.surname.trim() || !draft.purchaseCost.trim()) {
			updateMachineDraft(id, { error: 'Renseignez le modèle, le surnom et le prix d’achat.' });
			return;
		}

		updateMachineDraft(id, { busy: true, error: '' });
		try {
			const result = await gql<{ createMachine: Machine }>(
				`mutation($input: MachineInput!) {
					createMachine(input: $input) {
						id surname modelId purchaseCost { value unit } printingTime { value unit } state
						model {
							id brand name
							maintenanceCost { value numeratorUnit denominatorUnit }
							lifetime { value unit }
							averagePower { value unit }
						}
					}
				}`,
				{
					input: {
						modelId: draft.modelId,
						surname: draft.surname.trim(),
						purchaseCost: { value: draft.purchaseCost, unit: '€' },
						printingTime: { value: '0', unit: 'h' },
						state: 'available'
					}
				}
			);

			updateMachineDraft(id, { animateRemoval: false });
			await tick();
			cancelMachineDraft(id);
			machines = [result.createMachine, ...machines].slice(0, machinePage.pageSize);
			pinnedMachineIds = [result.createMachine.id, ...pinnedMachineIds];
			const totalItems = machinePage.totalItems + 1;
			machinePage = {
				...machinePage,
				totalItems,
				totalPages: Math.ceil(totalItems / machinePage.pageSize)
			};
		} catch (cause) {
			updateMachineDraft(id, {
				busy: false,
				error: cause instanceof Error ? cause.message : 'Impossible de créer la machine.'
			});
		}
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
					state: 'available',
					purchaseCost: { value: machineForm.purchaseCost, unit: '€' },
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
			await gql(`mutation($input: MachineInput!) { createMachine(input: $input) { id } }`, {
				input: {
					...machineForm,
					state: 'available',
					purchaseCost: { value: machineForm.purchaseCost, unit: '€' },
					printingTime: { value: '0', unit: 'h' }
				}
			});
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
			[machine.id]: {
				modelId: machine.modelId,
				surname: machine.surname,
				state: machine.state,
				busy: false,
				error: ''
			}
		};
	}

	function cancelMachineEdit(id: string) {
		const remaining = { ...machineEdits };
		delete remaining[id];
		machineEdits = remaining;
	}

	function updateInlineMachineState(id: string, state: MachineState) {
		const edit = machineEdits[id];
		if (!edit) return;
		machineEdits = { ...machineEdits, [id]: { ...edit, state } };
	}

	async function saveInlineMachine(id: string) {
		const edit = machineEdits[id];
		if (!edit?.surname.trim()) return;
		machineEdits = { ...machineEdits, [id]: { ...edit, busy: true, error: '' } };
		try {
			await gql(
				`mutation($id: String!, $input: MachineChangeset!) { patchMachine(id: $id, input: $input) { id } }`,
				{ id, input: { modelId: edit.modelId, surname: edit.surname.trim(), state: edit.state } }
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

<svelte:window
	onbeforeunload={(event) => {
		if (!hasUnsavedTableRows) return;
		event.preventDefault();
	}}
/>

<svelte:head><title>Machines — Acebau</title></svelte:head>

<PageShell>
	<ModuleHeader title="Machines" />
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
			<Table loading loadingColumns={8} />
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
						<div class="grid grid-cols-6 gap-3">
							<label class="col-span-3 label max-[600px]:col-span-6"
								>Marque<Input
									required
									bind:value={modelForm.brand}
									placeholder="Ex. Prusa"
								/></label
							>
							<label class="col-span-3 label max-[600px]:col-span-6"
								>Nom<Input required bind:value={modelForm.name} placeholder="Ex. MK4" /></label
							>
							<div class="col-span-2 max-[600px]:col-span-6">
								<RatioInput
									label="Coût de maintenance"
									required
									requiredFeedback={RequiredFeedback.Full}
									bind:value={modelForm.maintenanceCostValue}
									numeratorUnit="€"
									bind:denominatorUnit={modelForm.maintenanceCostUnit}
									numeratorUnits={priceUnits}
									denominatorUnits={maintenanceCostUnits}
								/>
							</div>
							<div class="col-span-2 max-[600px]:col-span-6">
								<MetricInput
									label="Durée de vie"
									required
									requiredFeedback={RequiredFeedback.Full}
									kind="time"
									bind:value={modelForm.lifetimeValue}
									bind:unit={modelForm.lifetimeUnit}
									units={timeUnits}
								/>
							</div>
							<div class="col-span-2 max-[600px]:col-span-6">
								<MetricInput
									label="Puissance moyenne"
									required
									requiredFeedback={RequiredFeedback.Full}
									kind="power"
									bind:value={modelForm.averagePowerValue}
									bind:unit={modelForm.averagePowerUnit}
									units={powerUnits}
								/>
							</div>
						</div>
						<div class="flex justify-end border-t border-surface-300-700 pt-5">
							<Button type="submit" tone="tertiary">Continuer</Button>
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
							><MetricInput
								label="Prix d’achat réel"
								required
								requiredFeedback={RequiredFeedback.Full}
								bind:value={machineForm.purchaseCost}
								unit="€"
								units={priceUnits}
							/>
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
									>{machineStateLabel('available')} · {machineForm.purchaseCost}€</small
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
			class="mb-6 grid grid-cols-3 gap-3.5 max-[1000px]:grid-cols-2 max-[850px]:grid-cols-1"
			aria-label="Indicateurs machines"
		>
			<Kpi
				label="Machines utilisables"
				value={`${usableMachines} / ${machines.length}`}
				detail="Disponibles ou en production"
			/>
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
				<Button tone="tertiary" size="sm" onclick={addMachineDraft} disabled={models.length === 0}>
					<Plus size={16} />Nouvelle machine
				</Button>
			{/snippet}
			<Table
				responsiveCards
				loading={pageLoading}
				loadingColumns={8}
				page={machinePage.page}
				pageSize={machinePage.pageSize}
				totalItems={machinePage.totalItems}
				totalPages={machinePage.totalPages}
				onPageChange={loadMachinePage}
				class="min-w-[62rem] table-fixed"
			>
				<colgroup>
					<col class="w-24" />
					<col class="w-42" />
					<col class="w-40" />
					<col class="w-38" />
					<col class="w-32" />
					<col class="w-28" />
					<col class="w-40" />
					<col class="w-22" />
				</colgroup>
				<thead
					><tr
						><th>#</th><th>Machine</th><th>Modèle</th><th>État</th><th>Temps d’impression</th><th
							>Coût effectif</th
						><th>Prochaine maintenance</th><th><span class="sr-only">Actions</span></th></tr
					></thead
				>
				<tbody>
					{#each machineDrafts as draft (draft.id)}
						<TableDraftRow animateRemoval={draft.animateRemoval}>
							<td data-label="" class="mobile-card-hidden">
								<Badge small tonal tertiary>Nouveau</Badge>
							</td>
							<td data-label="Machine">
								<div class="grid gap-2">
									<div class="flex items-center">
										<Input
											required
											requiredFeedback={RequiredFeedback.None}
											invalid={Boolean(draft.error && !draft.surname.trim())}
											validationAttempt={draft.validationAttempt}
											bind:value={draft.surname}
											placeholder="Ex. K2 du fond"
											aria-label="Surnom de la nouvelle machine"
											disabled={draft.busy}
										/>
									</div>
								</div>
							</td>
							<td data-label="Modèle">
								<div class="flex items-center">
									<Input
										as="select"
										required
										requiredFeedback={RequiredFeedback.None}
										invalid={Boolean(draft.error && !draft.modelId)}
										validationAttempt={draft.validationAttempt}
										bind:value={draft.modelId}
										aria-label="Modèle de la nouvelle machine"
										disabled={draft.busy}
									>
										{#each models as model (model.id)}
											<option value={model.id}>{model.brand} · {model.name}</option>
										{/each}
									</Input>
								</div>
							</td>
							<td data-label="État"><MachineStatus state="available" /></td>
							<td data-label="Temps d’impression">0h</td>
							<td data-label="Prix d’achat">
								<div class="flex min-w-28 items-center gap-1.5">
									<Input
										required
										requiredFeedback={RequiredFeedback.None}
										invalid={Boolean(draft.error && !draft.purchaseCost.trim())}
										validationAttempt={draft.validationAttempt}
										bind:value={draft.purchaseCost}
										placeholder="0,00"
										inputmode="decimal"
										aria-label="Prix d’achat de la nouvelle machine"
										disabled={draft.busy}
									/><span class="font-medium">€</span>
								</div>
							</td>
							<td data-label="Prochaine maintenance">{hardcodedNextMaintenance}</td>
							<td data-label="" class="mobile-card-actions">
								<IconAction
									label="Créer la machine"
									tone="success"
									disabled={draft.busy}
									onclick={() => saveMachineDraft(draft.id)}><Check size={18} /></IconAction
								>
								<IconAction
									label="Annuler la création"
									tone="error"
									disabled={draft.busy}
									onclick={() => cancelMachineDraft(draft.id)}><X size={18} /></IconAction
								>
							</td>
						</TableDraftRow>
					{/each}
					{#each machines as machine, index (machine.id)}
						<tr
							class="transition hover:bg-surface-100-900"
							class:bg-tertiary-50-950={pinnedMachineIds.includes(machine.id)}
						>
							<td data-label="" class="mobile-card-hidden text-surface-700-300">
								{(machinePage.page - 1) * machinePage.pageSize + index + 1}
							</td>
							<td data-label="Machine">
								{#if machineEdits[machine.id]}
									<div class="grid gap-2">
										<Input
											bind:value={machineEdits[machine.id].surname}
											class="w-full min-w-0"
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
							<td data-label="Modèle">
								{#if machineEdits[machine.id]}
									<Input
										as="select"
										bind:value={machineEdits[machine.id].modelId}
										class="w-full min-w-0"
										disabled={machineEdits[machine.id].busy}
										aria-label="Modèle de la machine"
									>
										{#each models as model (model.id)}
											<option value={model.id}>{model.brand} · {model.name}</option>
										{/each}
									</Input>
								{:else}
									<strong>{machine.model?.name ?? 'Modèle indisponible'}</strong>
									<small>{machine.model?.brand ?? '—'}</small>
								{/if}
							</td>
							<td data-label="État"
								>{#if machineEdits[machine.id]}<SelectableBadge
										value={machineEdits[machine.id].state}
										options={machineStateOptions}
										size="sm"
										disabled={machineEdits[machine.id].busy}
										ariaLabel={`Modifier l’état : ${machineStateLabel(machineEdits[machine.id].state)}`}
										onChange={(state) => updateInlineMachineState(machine.id, state)}
									/>
								{:else}<MachineStatus state={machine.state} />{/if}</td
							>
							<td data-label="Temps d’impression">{metricLabel(machine.printingTime)}</td>
							<td data-label="Coût effectif">{hardcodedPowerCost}</td>
							<td data-label="Prochaine maintenance">{hardcodedNextMaintenance}</td>
							<td data-label="" class="mobile-card-actions">
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
									>{metricLabel(model.averagePower)} · durée de vie {metricLabel(
										model.lifetime
									)}{#if machineCount > 0}
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
					<div class="grid grid-cols-6 gap-3">
						<label class="col-span-3 label max-[600px]:col-span-6"
							>Marque<Input required bind:value={modelForm.brand} /></label
						><label class="col-span-3 label max-[600px]:col-span-6"
							>Nom<Input required bind:value={modelForm.name} /></label
						>
						<div class="col-span-2 max-[600px]:col-span-6">
							<RatioInput
								label="Coût de maintenance"
								required
								requiredFeedback={RequiredFeedback.Full}
								bind:value={modelForm.maintenanceCostValue}
								numeratorUnit="€"
								bind:denominatorUnit={modelForm.maintenanceCostUnit}
								numeratorUnits={priceUnits}
								denominatorUnits={maintenanceCostUnits}
							/>
						</div>
						<div class="col-span-2 max-[600px]:col-span-6">
							<MetricInput
								label="Durée de vie"
								required
								requiredFeedback={RequiredFeedback.Full}
								kind="time"
								bind:value={modelForm.lifetimeValue}
								bind:unit={modelForm.lifetimeUnit}
								units={timeUnits}
							/>
						</div>
						<div class="col-span-2 max-[600px]:col-span-6">
							<MetricInput
								label="Puissance moyenne"
								required
								requiredFeedback={RequiredFeedback.Full}
								kind="power"
								bind:value={modelForm.averagePowerValue}
								bind:unit={modelForm.averagePowerUnit}
								units={powerUnits}
							/>
						</div>
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
			contentClasses="w-full max-w-3xl rounded-container border border-surface-300-700 bg-surface-50-950 p-7 shadow-xl max-[600px]:p-5"
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
					<div class="grid grid-cols-3 gap-3 max-[700px]:grid-cols-1">
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
						><MetricInput
							label="Prix d’achat réel"
							required
							requiredFeedback={RequiredFeedback.Full}
							bind:value={machineForm.purchaseCost}
							unit="€"
							units={priceUnits}
						/>
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
