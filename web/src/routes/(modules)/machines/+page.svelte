<script lang="ts">
	import { goto, invalidateAll } from '$app/navigation';
	import { tick } from 'svelte';
	import Decimal from 'decimal.js';
	import Check from '@lucide/svelte/icons/check';
	import Plus from '@lucide/svelte/icons/plus';
	import Printer from '@lucide/svelte/icons/printer';
	import Pencil from '@lucide/svelte/icons/pencil';
	import Trash2 from '@lucide/svelte/icons/trash-2';
	import Settings from '@lucide/svelte/icons/settings';
	import Wrench from '@lucide/svelte/icons/wrench';
	import X from '@lucide/svelte/icons/x';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import DataSourceNotice from '$lib/components/ui/DataSourceNotice.svelte';
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import IconAction from '$lib/components/ui/IconAction.svelte';
	import Input from '$lib/components/ui/forms/Input.svelte';
	import Checkbox from '$lib/components/ui/forms/Checkbox.svelte';
	import Modal from '$lib/components/ui/Modal.svelte';
	import MachineStatus from '$lib/components/machines/MachineStatus.svelte';
	import MachineMaintenanceOverview from '$lib/components/machines/MachineMaintenanceOverview.svelte';
	import MachineMaintenanceHistory from '$lib/components/machines/MachineMaintenanceHistory.svelte';
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
	import {
		maintenanceKindLabels,
		type MachineMaintenanceKind,
		type MachineMaintenancePage,
		type MachineMaintenanceSetting,
		type MachineMaintenanceStatus,
		type Machine,
		type MachineState,
		type MachineModel,
		type MachinePage
	} from '$lib/machine';
	import {
		Metric,
		Ratio,
		type MetricDTO,
		type PowerUnit,
		type PriceUnit,
		type RatioDTO,
		type TimeUnit
	} from '$lib/unit';
	import type { PageProps } from './$types';

	const machineStateOptions = [
		{ value: 'available', label: 'Disponible', tone: 'success' },
		{ value: 'running', label: 'En production', tone: 'secondary' },
		{ value: 'maintenance', label: 'Maintenance', tone: 'warning' },
		{ value: 'broken', label: 'En panne', tone: 'error' }
	] satisfies ReadonlyArray<{ value: MachineState; label: string; tone: string }>;

	type ModelForm = {
		brand: string;
		name: string;
		maintenanceCostValue: string;
		lifetimeValue: string;
		averagePowerValue: string;
		hasCarbonFilter: boolean;
	};

	type MachineForm = {
		modelId: string;
		surname: string;
		purchaseCost: string;
		nozzleSize: string;
	};
	type ModelRowForm = ModelForm & {
		busy: boolean;
		error: string;
		validationAttempt: number;
	};

	let { data }: PageProps = $props();
	let models = $state<MachineModel[]>([]);
	let machines = $state<Machine[]>([]);
	let modelMachineCounts = $state<Record<string, number>>({});
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
	type ModalKind =
		| 'new-model'
		| 'new-machine'
		| 'machine-settings'
		| 'record-maintenance'
		| 'confirm-delete-machine'
		| null;
	let activeModal = $state<ModalKind>(null);
	let onboardingStep = $state<1 | 2 | 3>(1);
	let selectedMachineId = $state<string | null>(null);
	let modelForm = $state<ModelForm>(emptyModel());
	let machineForm = $state<MachineForm>(emptyMachine());
	let machineEdits = $state<
		Record<
			string,
			{
				modelId: string;
				surname: string;
				nozzleSize: string;
				state: MachineState;
				busy: boolean;
				error: string;
			}
		>
	>({});
	let modelEdits = $state<Record<string, ModelRowForm>>({});
	let maintenanceSettings = $state<MachineMaintenanceSetting[]>([]);
	let maintenanceHistory = $state<MachineMaintenancePage>({
		items: [],
		page: 1,
		pageSize: 10,
		totalItems: 0,
		totalPages: 0
	});
	let maintenanceStatuses = $state<Record<string, MachineMaintenanceStatus[]>>({});
	let maintenanceKinds = $state<MachineMaintenanceKind[]>([]);
	let maintenanceNotes = $state('');
	let maintenancePerformedAt = $state('');
	let maintenanceForm = $state<Record<string, { due: string; critical: string }>>({});
	let hasUnsavedTableRows = $derived(
		Object.keys(machineEdits).length > 0 || Object.keys(modelEdits).length > 0
	);

	$effect(() => {
		models = data.models;
		machines = data.machines;
		modelMachineCounts = data.modelMachineCounts;
		maintenanceSettings = data.maintenanceSettings;
		maintenanceHistory = data.maintenanceHistory;
		maintenanceStatuses = data.maintenanceStatuses;
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
	const onboardingSteps = ['Créer un modèle', 'Ajouter une machine', 'Vérifier les informations'];
	const timeUnits = [
		{ value: 'min', label: 'minute' },
		{ value: 'h', label: 'heure' },
		{ value: 'd', label: 'jour' },
		{ value: 'mo', label: 'mois' },
		{ value: 'y', label: 'année' }
	];
	const priceUnits = [{ value: '€', label: '€' }];
	const yearUnits = [{ value: 'y', label: 'année' }];
	const wattUnits = [{ value: 'W', label: 'W' }];
	const millimeterUnits = [{ value: 'mm', label: 'mm' }];

	function emptyModel(): ModelForm {
		return {
			brand: '',
			name: '',
			maintenanceCostValue: '5',
			lifetimeValue: '10',
			averagePowerValue: '100',
			hasCarbonFilter: false
		};
	}

	function metricLabel(metric: MetricDTO) {
		return `${metric.value}${metric.unit}`;
	}

	function nozzleLabel(nozzleSize: MetricDTO) {
		const millimeters = Metric.from(nozzleSize).convertTo('mm').value.toFixed(1).replace('.', ',');
		return `${millimeters} mm`;
	}

	function nozzleValueLabel(value: string) {
		try {
			return `${new Decimal(value).toFixed(1).replace('.', ',')} mm`;
		} catch {
			return `${value} mm`;
		}
	}

	function ratioLabel(ratio: RatioDTO<PriceUnit, TimeUnit>) {
		const yearly = Ratio.from(ratio).convertTo('€', 'y');
		return `${yearly.value.toDecimalPlaces(2).toString()}€/année`;
	}

	function lifetimeLabel(lifetime: MetricDTO<TimeUnit>) {
		const yearly = Metric.from(lifetime).convertTo('y');
		return `${yearly.value.toDecimalPlaces(2).toString()} ans`;
	}

	function powerLabel(power: MetricDTO<PowerUnit>) {
		const watts = Metric.from(power).convertTo('W');
		return `${watts.value.toDecimalPlaces(2).toString()}W`;
	}

	function metricUsageCostLabel(cost: RatioDTO<PriceUnit, TimeUnit>) {
		const hourly = Ratio.from(cost).convertTo('€', 'h');
		return `${hourly.value.toDecimalPlaces(4).toString()}€/h`;
	}

	function modelInput(form: ModelForm) {
		return {
			brand: form.brand,
			name: form.name,
			maintenanceCost: {
				value: form.maintenanceCostValue,
				numeratorUnit: '€',
				denominatorUnit: 'y'
			},
			lifetime: { value: form.lifetimeValue, unit: 'y' },
			averagePower: { value: form.averagePowerValue, unit: 'W' },
			hasCarbonFilter: form.hasCarbonFilter
		};
	}

	function emptyMachine(): MachineForm {
		return { modelId: '', surname: '', purchaseCost: '', nozzleSize: '0.4' };
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
			machineEdits = {};
			const result = await gql<{ machines: MachinePage }>(
				`query MachineTablePage($page: Int!, $pageSize: Int!) {
					machines(page: $page, pageSize: $pageSize) {
						items {
							id surname modelId purchaseCost { value unit } nozzleSize { value unit } printingTime { value unit }
							usageCost { value numeratorUnit denominatorUnit } state
							model {
								id brand name
								maintenanceCost { value numeratorUnit denominatorUnit }
								lifetime { value unit }
								averagePower { value unit }
								hasCarbonFilter
							}
						}
						page pageSize totalItems totalPages
					}
				}`,
				{ page, pageSize: 10 }
			);
			machines = result.machines.items;
			const overview = await gql<{
				machineMaintenanceOverview: Array<{
					machineId: string;
					statuses: MachineMaintenanceStatus[];
				}>;
			}>(
				`query($machineIds: [String!]!) {
					machineMaintenanceOverview(machineIds: $machineIds) {
						machineId
						statuses {
							kind criticity lastPerformedAt
							dueAfter { value unit }
							criticalAfter { value unit }
							printingTimeSinceMaintenance { value unit }
						}
					}
				}`,
				{ machineIds: machines.map((machine) => machine.id) }
			);
			maintenanceStatuses = Object.fromEntries(
				overview.machineMaintenanceOverview.map((entry) => [entry.machineId, entry.statuses])
			);
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

	async function loadMaintenanceHistoryPage(page: number) {
		const result = await gql<{ machineMaintenanceHistory: MachineMaintenancePage }>(
			`query($page: Int!, $pageSize: Int!) {
				machineMaintenanceHistory(page: $page, pageSize: $pageSize) {
					items { id machineId kind performedAt printingTime { value unit } notes machine { id surname } }
					page pageSize totalItems totalPages
				}
			}`,
			{ page, pageSize: maintenanceHistory.pageSize }
		);
		maintenanceHistory = result.machineMaintenanceHistory;
	}

	function openModelForm() {
		modelForm = emptyModel();
		error = '';
		activeModal = 'new-model';
	}

	function modelRowForm(model: MachineModel): ModelRowForm {
		const yearlyMaintenanceCost = Ratio.from(model.maintenanceCost).convertTo('€', 'y');
		const yearlyLifetime = Metric.from(model.lifetime).convertTo('y');
		const wattPower = Metric.from(model.averagePower).convertTo('W');
		return {
			brand: model.brand,
			name: model.name,
			maintenanceCostValue: yearlyMaintenanceCost.value.toDecimalPlaces(2).toString(),
			lifetimeValue: yearlyLifetime.value.toDecimalPlaces(2).toString(),
			averagePowerValue: wattPower.value.toDecimalPlaces(2).toString(),
			hasCarbonFilter: model.hasCarbonFilter,
			busy: false,
			error: '',
			validationAttempt: 0
		};
	}

	function beginModelEdit(model: MachineModel) {
		modelEdits = { ...modelEdits, [model.id]: modelRowForm(model) };
	}

	function updateModelEdit(id: string, changes: Partial<ModelRowForm>) {
		const edit = modelEdits[id];
		if (edit) modelEdits = { ...modelEdits, [id]: { ...edit, ...changes } };
	}

	function cancelModelEdit(id: string) {
		const remaining = { ...modelEdits };
		delete remaining[id];
		modelEdits = remaining;
	}

	function openMachineForm() {
		machineForm = { ...emptyMachine(), modelId: models[0]?.id ?? '' };
		activeModal = 'new-machine';
	}

	function requestDeleteMachine(machine: Machine) {
		selectedMachineId = machine.id;
		activeModal = 'confirm-delete-machine';
	}

	function openMachineRow(event: MouseEvent | KeyboardEvent, machine: Machine) {
		if (machineEdits[machine.id]) return;
		if (event.target instanceof Element && event.target.closest('a, button, input, select')) return;
		void goto(`/machines/${machine.id}`);
	}

	function closeModal() {
		activeModal = null;
		onboardingStep = 1;
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

	function openMachineSettings() {
		maintenanceForm = Object.fromEntries(
			maintenanceSettings.map((setting) => [
				setting.id,
				{
					due: Metric.from(setting.dueAfter).convertTo('h').value.toDecimalPlaces(1).toString(),
					critical: Metric.from(setting.criticalAfter)
						.convertTo('h')
						.value.toDecimalPlaces(1)
						.toString()
				}
			])
		);
		activeModal = 'machine-settings';
	}

	function openMaintenanceRecorder(machine: Machine) {
		selectedMachineId = machine.id;
		maintenanceKinds = [];
		maintenanceNotes = '';
		maintenancePerformedAt = new Date().toISOString().slice(0, 16);
		activeModal = 'record-maintenance';
	}

	async function saveMaintenanceSettings() {
		saving = true;
		error = '';
		if (
			maintenanceSettings.some((setting) => {
				const form = maintenanceForm[setting.id];
				if (!form) return true;
				try {
					const due = new Decimal(form.due);
					const critical = new Decimal(form.critical);
					return due.lte(0) || critical.lt(due);
				} catch {
					return true;
				}
			})
		) {
			error = 'L’échéance critique doit être supérieure ou égale à l’échéance à prévoir.';
			saving = false;
			return;
		}
		try {
			await Promise.all(
				maintenanceSettings.map((setting) => {
					const form = maintenanceForm[setting.id];
					return gql(
						`mutation($id: String!, $input: MachineMaintenanceSettingChangeset!) {
							patchMachineMaintenanceSetting(id: $id, input: $input) { id }
						}`,
						{
							id: setting.id,
							input: {
								dueAfter: { value: form.due, unit: 'h' },
								criticalAfter: { value: form.critical, unit: 'h' }
							}
						}
					);
				})
			);
			closeModal();
			await invalidateAll();
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Impossible d’enregistrer les paramètres.';
		} finally {
			saving = false;
		}
	}

	async function recordMaintenance() {
		if (!selectedMachine || maintenanceKinds.length === 0 || !maintenancePerformedAt) return;
		saving = true;
		error = '';
		try {
			await Promise.all(
				maintenanceKinds.map((kind) =>
					gql(
						`mutation($input: MachineMaintenanceInput!) {
							createMachineMaintenance(input: $input) { id }
						}`,
						{
							input: {
								machineId: selectedMachine.id,
								kind,
								performedAt: new Date(maintenancePerformedAt).toISOString(),
								printingTime: selectedMachine.printingTime,
								notes: maintenanceNotes.trim() || null
							}
						}
					)
				)
			);
			closeModal();
			await invalidateAll();
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Impossible d’enregistrer la maintenance.';
		} finally {
			saving = false;
		}
	}

	async function saveModelDraft(id: string) {
		const draft = modelDrafts.find((candidate) => candidate.id === id);
		if (!draft || draft.busy) return;
		updateModelDraft(id, { validationAttempt: draft.validationAttempt + 1 });
		if (!draft.brand.trim() || !draft.name.trim()) {
			updateModelDraft(id, { error: 'Renseignez la marque et le nom.' });
			return;
		}

		updateModelDraft(id, { busy: true, error: '' });
		try {
			const result = await gql<{ createMachineModel: MachineModel }>(
				`mutation($input: MachineModelInput!) {
					createMachineModel(input: $input) {
						id brand name
						maintenanceCost { value numeratorUnit denominatorUnit }
						lifetime { value unit }
						averagePower { value unit }
						hasCarbonFilter
					}
				}`,
				{ input: modelInput(draft) }
			);
			updateModelDraft(id, { animateRemoval: false });
			await tick();
			cancelModelDraft(id);
			models = [result.createMachineModel, ...models];
		} catch (cause) {
			updateModelDraft(id, {
				busy: false,
				error: cause instanceof Error ? cause.message : 'Impossible de créer le modèle.'
			});
		}
	}

	async function saveModelEdit(id: string) {
		const edit = modelEdits[id];
		if (!edit || edit.busy) return;
		updateModelEdit(id, { validationAttempt: edit.validationAttempt + 1 });
		if (!edit.brand.trim() || !edit.name.trim()) {
			updateModelEdit(id, { error: 'Renseignez la marque et le nom.' });
			return;
		}

		updateModelEdit(id, { busy: true, error: '' });
		try {
			const result = await gql<{ updateMachineModel: MachineModel }>(
				`mutation($id: String!, $input: MachineModelInput!) {
					updateMachineModel(id: $id, input: $input) {
						id brand name
						maintenanceCost { value numeratorUnit denominatorUnit }
						lifetime { value unit }
						averagePower { value unit }
						hasCarbonFilter
					}
				}`,
				{ id, input: modelInput(edit) }
			);
			const updatedModel = result.updateMachineModel;
			models = models.map((model) => (model.id === id ? updatedModel : model));
			machines = machines.map((machine) =>
				machine.modelId === id ? { ...machine, model: updatedModel } : machine
			);
			cancelModelEdit(id);
		} catch (cause) {
			updateModelEdit(id, {
				busy: false,
				error: cause instanceof Error ? cause.message : 'Impossible de modifier le modèle.'
			});
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
					nozzleSize: { value: machineForm.nozzleSize, unit: 'mm' },
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
					nozzleSize: { value: machineForm.nozzleSize, unit: 'mm' },
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
		const machineCount = modelMachineCounts[id] ?? 0;
		if (machineCount > 0) {
			error = `Ce modèle ne peut pas être supprimé : ${machineCount} machine${machineCount > 1 ? 's lui sont' : ' lui est'} encore associée${machineCount > 1 ? 's' : ''}.`;
			return;
		}
		saving = true;
		error = '';
		try {
			await gql(`mutation($id: String!) { deleteMachineModel(id: $id) }`, { id });
			cancelModelEdit(id);
			models = models.filter((model) => model.id !== id);
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
			const deletedMachine = machines.find((machine) => machine.id === deletedMachineId);
			machines = machines.filter((machine) => machine.id !== deletedMachineId);
			if (deletedMachine) {
				modelMachineCounts = {
					...modelMachineCounts,
					[deletedMachine.modelId]: Math.max(
						0,
						(modelMachineCounts[deletedMachine.modelId] ?? 1) - 1
					)
				};
			}
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
				nozzleSize: Metric.from(machine.nozzleSize).convertTo('mm').value.toString(),
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
		if (edit) machineEdits = { ...machineEdits, [id]: { ...edit, state } };
	}

	async function saveInlineMachine(id: string) {
		const edit = machineEdits[id];
		if (!edit?.surname.trim() || !edit.nozzleSize.trim()) return;
		machineEdits = { ...machineEdits, [id]: { ...edit, busy: true, error: '' } };
		try {
			await gql(
				`mutation($id: String!, $input: MachineChangeset!) { patchMachine(id: $id, input: $input) { id } }`,
				{
					id,
					input: {
						modelId: edit.modelId,
						surname: edit.surname.trim(),
						nozzleSize: { value: edit.nozzleSize, unit: 'mm' },
						state: edit.state
					}
				}
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
	<ModuleHeader title="Machines">
		{#snippet actions()}
			<Button variant="outlined" tone="surface" onclick={openMachineSettings}>
				<Settings size={17} />Paramètres
			</Button>
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
									numeratorUnits={priceUnits}
									denominatorUnits={yearUnits}
								/>
							</div>
							<div class="col-span-2 max-[600px]:col-span-6">
								<MetricInput
									label="Durée de vie"
									required
									requiredFeedback={RequiredFeedback.Full}
									bind:value={modelForm.lifetimeValue}
									units={yearUnits}
								/>
							</div>
							<div class="col-span-2 max-[600px]:col-span-6">
								<MetricInput
									label="Puissance moyenne"
									required
									requiredFeedback={RequiredFeedback.Full}
									bind:value={modelForm.averagePowerValue}
									units={wattUnits}
								/>
							</div>
							<label class="col-span-6 flex items-center gap-2 text-sm text-surface-700-300">
								<Checkbox bind:checked={modelForm.hasCarbonFilter} />
								Filtre à charbon
							</label>
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
						<div class="grid grid-cols-3 gap-3 max-[600px]:grid-cols-1">
							<label class="label"
								>Surnom<Input
									required
									bind:value={machineForm.surname}
									placeholder="Ex. K2 du fond"
								/></label
							><MetricInput
								label="Prix d’achat"
								required
								requiredFeedback={RequiredFeedback.Full}
								bind:value={machineForm.purchaseCost}
								unit="€"
								units={priceUnits}
							/>
							<MetricInput
								label="Buse"
								required
								requiredFeedback={RequiredFeedback.Full}
								bind:value={machineForm.nozzleSize}
								unit="mm"
								units={millimeterUnits}
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
									>{modelForm.averagePowerValue}W · durée de vie {modelForm.lifetimeValue} ans</small
								>
							</div>
							<div class="rounded-base border border-surface-300-700 bg-surface-100-900 p-4">
								<p class="mb-2 text-xs font-bold tracking-wider text-surface-700-300 uppercase">
									Première machine
								</p>
								<strong class="block text-base text-surface-900-100">{machineForm.surname}</strong
								><small class="text-surface-700-300"
									>{machineStateLabel('available')} · buse {nozzleValueLabel(machineForm.nozzleSize)} · {machineForm.purchaseCost}€</small
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
		<div class="flex flex-col gap-6">
			<section
				class="grid grid-cols-3 gap-3.5 max-[1000px]:grid-cols-2 max-[850px]:grid-cols-1"
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
					<Button variant="outlined" tone="surface" size="sm" onclick={openModelManager}>
						Voir les modèles
					</Button>
					<Button
						tone="tertiary"
						size="sm"
						onclick={openMachineForm}
						disabled={models.length === 0}
					>
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
					class="min-w-[64rem] table-fixed"
				>
					<colgroup>
						<col class="w-16" />
						<col class="w-36" />
						<col class="w-40" />
						<col class="w-28" />
						<col class="w-32" />
						<col class="w-32" />
						<col class="w-32" />
						<col class="w-40" />
					</colgroup>
					<thead
						><tr
							><th>#</th><th>Machine</th><th>Modèle</th><th>Buse</th><th>État</th><th>Temps d’impression</th><th
								>Coût d’usage</th
							><th><span class="sr-only">Actions</span></th></tr
						></thead
					>
					<tbody>
						{#each machines as machine, index (machine.id)}
							<tr
								class="cursor-pointer transition hover:bg-surface-100-900"
								role="link"
								tabindex="0"
								aria-label={`Ouvrir ${machine.surname}`}
								onclick={(event) => openMachineRow(event, machine)}
								onkeydown={(event) => {
									if (event.key === 'Enter') openMachineRow(event, machine);
								}}
							>
								<td data-label="" class="mobile-card-hidden text-surface-700-300">
									{(machinePage.page - 1) * machinePage.pageSize + index + 1}
								</td>
								<td data-label="Machine">
									{#if machineEdits[machine.id]}
										<div class="grid gap-2">
											<Input
												bind:value={machineEdits[machine.id].surname}
												aria-label="Surnom de la machine"
												disabled={machineEdits[machine.id].busy}
											/>
											{#if machineEdits[machine.id].error}<small class="text-error-600-400" role="alert"
													>{machineEdits[machine.id].error}</small
												>{/if}
										</div>
									{:else}
										<strong>{machine.surname}</strong><small>#{machine.id.slice(0, 8)}</small>
									{/if}
								</td>
								<td data-label="Modèle">
									{#if machineEdits[machine.id]}
										<Input
											as="select"
											bind:value={machineEdits[machine.id].modelId}
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
								<td data-label="Buse">
									{#if machineEdits[machine.id]}
										<MetricInput
											label={`Diamètre de buse de ${machine.surname}`}
											labelVisible={false}
											required
											bind:value={machineEdits[machine.id].nozzleSize}
											unit="mm"
											units={millimeterUnits}
											disabled={machineEdits[machine.id].busy}
										/>
									{:else}
										{nozzleLabel(machine.nozzleSize)}
									{/if}
								</td>
								<td data-label="État">
									{#if machineEdits[machine.id]}
										<SelectableBadge
											value={machineEdits[machine.id].state}
											options={machineStateOptions}
											size="sm"
											disabled={machineEdits[machine.id].busy}
											ariaLabel={`Modifier l’état : ${machineStateLabel(machineEdits[machine.id].state)}`}
											onChange={(state) => updateInlineMachineState(machine.id, state)}
										/>
									{:else}
										<MachineStatus state={machine.state} />
									{/if}
								</td>
								<td data-label="Temps d’impression">{metricLabel(machine.printingTime)}</td>
								<td data-label="Coût d’usage">{metricUsageCostLabel(machine.usageCost)}</td>
								<td data-label="" class="mobile-card-actions text-right whitespace-nowrap">
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
											label={`Enregistrer une maintenance pour ${machine.surname}`}
											onclick={() => openMaintenanceRecorder(machine)}
											><Wrench size={18} /></IconAction
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

			<div
				class="grid grid-cols-[minmax(18rem,0.8fr)_minmax(0,1.7fr)] items-start gap-5 max-[900px]:grid-cols-1"
			>
				<MachineMaintenanceOverview {machines} statuses={maintenanceStatuses} />
				<TableSection
					title="Historique de maintenance"
					description="Les dernières opérations enregistrées sur le parc."
				>
					<MachineMaintenanceHistory
						records={maintenanceHistory}
						showMachine
						onPageChange={loadMaintenanceHistoryPage}
					/>
				</TableSection>
			</div>
		</div>
	{/if}

	{#if activeModal === 'machine-settings'}
		<Modal
			open={true}
			onOpenChange={handleModalChange}
			contentClasses="w-full max-w-2xl rounded-container border border-surface-300-700 bg-surface-50-950 p-7 shadow-xl max-[600px]:p-5"
		>
			{#snippet content()}
				<div class="mb-6 flex items-start justify-between gap-4">
					<div>
						<h2 class="text-2xl font-semibold text-surface-900-100">Paramètres de maintenance</h2>
						<p class="mt-1 text-sm text-surface-700-300">
							Intervalles communs à toutes les machines, exprimés en heures d’impression.
						</p>
					</div>
					<button
						class="btn-icon preset-tonal-surface"
						type="button"
						aria-label="Fermer"
						onclick={closeModal}><X size={18} /></button
					>
				</div>
				{#if error}<p class="mb-4 rounded-base preset-tonal-error p-3" role="alert">{error}</p>{/if}
				<div class="grid gap-3">
					<div
						class="grid grid-cols-[minmax(0,1fr)_9rem_9rem] gap-3 px-1 text-xs font-semibold tracking-wide text-surface-700-300 uppercase max-[600px]:hidden"
					>
						<span>Opération</span><span>À prévoir</span><span>Critique</span>
					</div>
					{#each maintenanceSettings as setting (setting.id)}
						<div
							class="grid grid-cols-[minmax(0,1fr)_9rem_9rem] items-end gap-3 rounded-base bg-surface-100-900 p-3 max-[600px]:grid-cols-2"
						>
							<strong class="self-center max-[600px]:col-span-2"
								>{maintenanceKindLabels[setting.kind]}</strong
							>
							<MetricInput
								label="À prévoir"
								labelVisible={false}
								required
								bind:value={maintenanceForm[setting.id].due}
								units={[{ value: 'h', label: 'h' }]}
							/>
							<MetricInput
								label="Critique"
								labelVisible={false}
								required
								bind:value={maintenanceForm[setting.id].critical}
								units={[{ value: 'h', label: 'h' }]}
							/>
						</div>
					{/each}
				</div>
				<div class="mt-6 flex justify-end gap-2 border-t border-surface-300-700 pt-5">
					<Button variant="outlined" tone="surface" onclick={closeModal}>Annuler</Button>
					<Button tone="tertiary" disabled={saving} onclick={saveMaintenanceSettings}
						>Enregistrer</Button
					>
				</div>
			{/snippet}
		</Modal>
	{/if}

	{#if activeModal === 'record-maintenance' && selectedMachine}
		<Modal
			open={true}
			onOpenChange={handleModalChange}
			contentClasses="w-full max-w-xl rounded-container border border-surface-300-700 bg-surface-50-950 p-7 shadow-xl max-[600px]:p-5"
		>
			{#snippet content()}
				<div class="mb-6 flex items-start justify-between gap-4">
					<div>
						<h2 class="text-2xl font-semibold text-surface-900-100">Enregistrer une maintenance</h2>
						<p class="mt-1 text-sm text-surface-700-300">
							{selectedMachine.surname} · compteur actuel {Metric.from(selectedMachine.printingTime)
								.convertTo('h')
								.value.toDecimalPlaces(1)} h
						</p>
					</div>
					<button
						class="btn-icon preset-tonal-surface"
						type="button"
						aria-label="Fermer"
						onclick={closeModal}><X size={18} /></button
					>
				</div>
				{#if error}<p class="mb-4 rounded-base preset-tonal-error p-3" role="alert">{error}</p>{/if}
				<fieldset>
					<legend class="mb-3 font-semibold">Opérations effectuées</legend>
					<div class="grid gap-2">
						{#each maintenanceSettings.filter((setting) => setting.kind !== 'carbonFilter' || selectedMachine.model.hasCarbonFilter) as setting (setting.kind)}
							<label
								class="flex min-h-11 items-center gap-3 rounded-base border border-surface-300-700 px-3 py-2 hover:bg-surface-100-900"
							>
								<Checkbox
									value={setting.kind}
									bind:group={maintenanceKinds}
								/>
								<span>{maintenanceKindLabels[setting.kind]}</span>
							</label>
						{/each}
					</div>
				</fieldset>
				<label class="label mt-5 block"
					>Date et heure<Input
						type="datetime-local"
						required
						bind:value={maintenancePerformedAt}
					/></label
				>
				<label class="label mt-5 block"
					>Notes facultatives<Input
						bind:value={maintenanceNotes}
						placeholder="Ex. Buse remplacée"
					/></label
				>
				<div class="mt-6 flex justify-end gap-2 border-t border-surface-300-700 pt-5">
					<Button variant="outlined" tone="surface" onclick={closeModal}>Annuler</Button>
					<Button
						tone="tertiary"
						disabled={saving || maintenanceKinds.length === 0 || !maintenancePerformedAt}
						onclick={recordMaintenance}>Enregistrer</Button
					>
				</div>
			{/snippet}
		</Modal>
	{/if}

	{#if activeModal === 'models'}
		<Modal
			open={true}
			onOpenChange={handleModalChange}
			contentClasses="max-h-[88vh] w-[min(94vw,82rem)] max-w-none overflow-y-auto rounded-container bg-surface-100-900 p-6 shadow-xl max-[600px]:p-4"
		>
			{#snippet content()}
				<div class="mb-4 flex items-start justify-between gap-4 px-1">
					<div>
						<div class="flex flex-wrap items-center gap-2.5">
							<h2 class="text-2xl font-semibold text-surface-900-100">Référentiel</h2>
							<Badge small tonal surface
								>{models.length} {models.length === 1 ? 'modèle' : 'modèles'}</Badge
							>
						</div>
						<p class="mt-1 text-sm text-surface-700-300">
							Gérez les caractéristiques communes à vos machines.
						</p>
					</div>
					<button
						class="btn-icon shrink-0 preset-tonal-surface"
						type="button"
						aria-label="Fermer"
						onclick={closeModal}><X size={18} /></button
					>
				</div>
				{#if error}<p class="mb-4 rounded-base preset-tonal-error p-3" role="alert">{error}</p>{/if}
				<TableSection title="Modèles de machines" class="shadow-md">
					{#snippet toolbar()}
						<Button tone="tertiary" size="sm" onclick={addModelDraft}>
							<Plus size={16} />Nouveau modèle
						</Button>
					{/snippet}
					<Table responsiveCards class="min-w-[80rem] table-fixed">
						<colgroup>
							<col class="w-24" /><col class="w-32" /><col class="w-36" /><col class="w-52" /><col
								class="w-44"
							/><col class="w-44" /><col class="w-24" /><col class="w-24" /><col class="w-24" />
						</colgroup>
						<thead>
							<tr>
								<th>#</th><th>Marque</th><th>Modèle</th><th>Maintenance</th><th>Durée de vie</th><th
									>Puissance</th
								><th>Filtre</th><th>Machines</th><th><span class="sr-only">Actions</span></th>
							</tr>
						</thead>
						<tbody>
							{#each modelDrafts as draft (draft.id)}
								<TableDraftRow animateRemoval={draft.animateRemoval}>
									<td data-label="" class="mobile-card-hidden"
										><Badge small tonal tertiary>Nouveau</Badge></td
									>
									<td data-label="Marque">
										<Input
											required
											requiredFeedback={RequiredFeedback.None}
											invalid={Boolean(draft.error && !draft.brand.trim())}
											validationAttempt={draft.validationAttempt}
											bind:value={draft.brand}
											disabled={draft.busy}
											aria-label="Marque du nouveau modèle"
										/>
									</td>
									<td data-label="Modèle">
										<Input
											required
											requiredFeedback={RequiredFeedback.None}
											invalid={Boolean(draft.error && !draft.name.trim())}
											validationAttempt={draft.validationAttempt}
											bind:value={draft.name}
											disabled={draft.busy}
											aria-label="Nom du nouveau modèle"
										/>
									</td>
									<td data-label="Maintenance"
										><RatioInput
											label="Coût de maintenance"
											labelVisible={false}
											required
											bind:value={draft.maintenanceCostValue}
											numeratorUnit="€"
											numeratorUnits={priceUnits}
											denominatorUnits={yearUnits}
										/></td
									>
									<td data-label="Durée de vie"
										><MetricInput
											label="Durée de vie"
											labelVisible={false}
											required
											bind:value={draft.lifetimeValue}
											units={yearUnits}
										/></td
									>
									<td data-label="Puissance"
										><MetricInput
											label="Puissance moyenne"
											labelVisible={false}
											required
											bind:value={draft.averagePowerValue}
											units={wattUnits}
										/></td
									>
									<td data-label="Filtre">
										<Checkbox
											bind:checked={draft.hasCarbonFilter}
											aria-label="Filtre à charbon"
										/>
									</td>
									<td data-label="Machines">—</td>
									<td data-label="" class="mobile-card-actions">
										<IconAction
											label="Créer le modèle"
											tone="success"
											disabled={draft.busy}
											onclick={() => saveModelDraft(draft.id)}><Check size={18} /></IconAction
										>
										<IconAction
											label="Annuler la création"
											tone="error"
											disabled={draft.busy}
											onclick={() => cancelModelDraft(draft.id)}><X size={18} /></IconAction
										>
									</td>
								</TableDraftRow>
							{/each}
							{#each models as model, index (model.id)}
								{@const machineCount = modelMachineCounts[model.id] ?? 0}
								<tr>
									<td data-label="" class="mobile-card-hidden text-surface-700-300">{index + 1}</td>
									{#if modelEdits[model.id]}
										<td data-label="Marque"
											><Input
												required
												requiredFeedback={RequiredFeedback.None}
												invalid={Boolean(
													modelEdits[model.id].error && !modelEdits[model.id].brand.trim()
												)}
												validationAttempt={modelEdits[model.id].validationAttempt}
												bind:value={modelEdits[model.id].brand}
												disabled={modelEdits[model.id].busy}
											/></td
										>
										<td data-label="Modèle"
											><Input
												required
												requiredFeedback={RequiredFeedback.None}
												invalid={Boolean(
													modelEdits[model.id].error && !modelEdits[model.id].name.trim()
												)}
												validationAttempt={modelEdits[model.id].validationAttempt}
												bind:value={modelEdits[model.id].name}
												disabled={modelEdits[model.id].busy}
											/></td
										>
										<td data-label="Maintenance"
											><RatioInput
												label="Coût de maintenance"
												labelVisible={false}
												required
												bind:value={modelEdits[model.id].maintenanceCostValue}
												numeratorUnit="€"
												numeratorUnits={priceUnits}
												denominatorUnits={yearUnits}
											/></td
										>
										<td data-label="Durée de vie"
											><MetricInput
												label="Durée de vie"
												labelVisible={false}
												required
												bind:value={modelEdits[model.id].lifetimeValue}
												units={yearUnits}
											/></td
										>
										<td data-label="Puissance"
											><MetricInput
												label="Puissance moyenne"
												labelVisible={false}
												required
												bind:value={modelEdits[model.id].averagePowerValue}
												units={wattUnits}
											/></td
										>
										<td data-label="Filtre">
											<Checkbox
												bind:checked={modelEdits[model.id].hasCarbonFilter}
												aria-label="Filtre à charbon"
											/>
										</td>
									{:else}
										<td data-label="Marque"><strong>{model.brand}</strong></td>
										<td data-label="Modèle"><strong>{model.name}</strong></td>
										<td data-label="Maintenance">{ratioLabel(model.maintenanceCost)}</td>
										<td data-label="Durée de vie">{lifetimeLabel(model.lifetime)}</td>
										<td data-label="Puissance">{powerLabel(model.averagePower)}</td>
										<td data-label="Filtre">{model.hasCarbonFilter ? 'Oui' : 'Non'}</td>
									{/if}
									<td data-label="Machines">{machineCount}</td>
									<td data-label="" class="mobile-card-actions">
										{#if modelEdits[model.id]}
											<IconAction
												label={`Confirmer la modification de ${model.name}`}
												tone="success"
												disabled={modelEdits[model.id].busy}
												onclick={() => saveModelEdit(model.id)}><Check size={18} /></IconAction
											>
											<IconAction
												label={`Annuler la modification de ${model.name}`}
												tone="error"
												disabled={modelEdits[model.id].busy}
												onclick={() => cancelModelEdit(model.id)}><X size={18} /></IconAction
											>
										{:else}
											<IconAction
												label={`Modifier ${model.name}`}
												onclick={() => beginModelEdit(model)}><Pencil size={18} /></IconAction
											>
											<IconAction
												label={machineCount > 0
													? `Impossible de supprimer ${model.name}, modèle utilisé`
													: `Supprimer ${model.name}`}
												tone="error"
												disabled={saving || machineCount > 0}
												onclick={() => deleteModel(model.id)}><Trash2 size={18} /></IconAction
											>
										{/if}
									</td>
								</tr>
							{:else}
								{#if modelDrafts.length === 0}<tr
										><td colspan="9" class="py-10 text-center text-surface-700-300"
											>Aucun modèle.</td
										></tr
									>{/if}
							{/each}
						</tbody>
					</Table>
				</TableSection>
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
					<div class="grid grid-cols-4 items-end gap-3 max-[700px]:grid-cols-1">
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
							label="Prix d’achat"
							required
							requiredFeedback={RequiredFeedback.Full}
							bind:value={machineForm.purchaseCost}
							unit="€"
							units={priceUnits}
						/>
						<MetricInput
							label="Buse"
							required
							requiredFeedback={RequiredFeedback.Full}
							bind:value={machineForm.nozzleSize}
							unit="mm"
							units={millimeterUnits}
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
