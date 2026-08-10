<script lang="ts">
	import ArrowRight from '@lucide/svelte/icons/arrow-right';
	import ClipboardList from '@lucide/svelte/icons/clipboard-list';
	import Factory from '@lucide/svelte/icons/factory';
	import FileWarning from '@lucide/svelte/icons/file-warning';
	import Package from '@lucide/svelte/icons/package';
	import Plus from '@lucide/svelte/icons/plus';
	import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
	import Truck from '@lucide/svelte/icons/truck';
	import Users from '@lucide/svelte/icons/users';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Kpi from '$lib/components/ui/Kpi.svelte';
	import { Tone } from '$lib/types';
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();

	type Action = {
		title: string;
		detail: string;
		module: string;
		tone: Tone;
		icon: typeof ClipboardList;
		href: string;
	};

	type ModuleSummary = {
		name: string;
		value: string;
		detail: string;
		action: string;
		tone: Tone;
		icon: typeof ClipboardList;
		href: string;
	};

	const iconMap: Record<string, typeof ClipboardList> = {
		orders: ClipboardList,
		invoices: FileWarning,
		production: Factory,
		inventory: Package,
		finance: TriangleAlert,
		resellers: Users
	};
	let actions = $derived(
		data.actions.map((action) => ({ ...action, icon: iconMap[action.icon] })) as Action[]
	);
	let summaries = $derived(
		data.summaries.map((summary) => ({
			...summary,
			icon: iconMap[summary.icon]
		})) as ModuleSummary[]
	);

	function softTone(tone: Tone): string {
		return {
			[Tone.Success]: 'bg-success-50-950 text-success-500',
			[Tone.Warning]: 'bg-warning-50-950 text-warning-500',
			[Tone.Error]: 'bg-error-50-950 text-error-500',
			[Tone.Secondary]: 'bg-secondary-50-950 text-secondary-500',
			[Tone.Tertiary]: 'bg-tertiary-50-950 text-tertiary-500',
			[Tone.Primary]: 'bg-primary-100-900 text-primary-500',
			[Tone.Surface]: 'bg-surface-100-900 text-surface-700-300'
		}[tone];
	}

	function dotTone(tone: Tone): string {
		return {
			[Tone.Success]: 'bg-success-500',
			[Tone.Warning]: 'bg-warning-500',
			[Tone.Error]: 'bg-error-500',
			[Tone.Secondary]: 'bg-secondary-500',
			[Tone.Tertiary]: 'bg-tertiary-500',
			[Tone.Primary]: 'bg-primary-500',
			[Tone.Surface]: 'bg-surface-500'
		}[tone];
	}
</script>

<svelte:head><title>Tableau de bord — Acebau</title></svelte:head>

<div class="mx-auto max-w-[1240px] px-12 py-10 pb-16 max-[850px]:px-5 max-[850px]:py-7">
	<header
		class="mb-8 flex items-end justify-between gap-6 max-[850px]:flex-col max-[850px]:items-start"
	>
		<div>
			<h1 class="m-0 text-4xl leading-tight font-bold tracking-tight text-surface-900-100">
				Bonjour Alexis
			</h1>
			<p class="mt-2 mb-0 text-surface-700-300">Voici ce qui mérite votre attention aujourd’hui.</p>
		</div>
		<Button tone="tertiary"><Plus size={17} />Nouvelle production</Button>
	</header>

	<section
		class="mb-6 grid grid-cols-4 gap-3.5 max-[1000px]:grid-cols-2 max-[650px]:grid-cols-1"
		aria-label="Vue exécutive du mois en cours"
	>
		<Kpi
			label="Chiffre d’affaires (mois en cours)"
			value="7 180 €"
			detail="+8 % versus le mois dernier"
		/>
		<Kpi label="Commandes à traiter" value="2" detail="842 € HT à accepter ou rejeter" />
		<Kpi
			label="Charge de production (7 j)"
			value="82 %"
			detail="71 h planifiées · 2 promesses menacées"
		/>
		<Kpi label="Alertes critiques" value="4" detail="Facture, stock et maintenance à surveiller" />
	</section>

	<section
		class="mb-6 card border border-surface-300-700 bg-surface-50-950 shadow-sm"
		aria-labelledby="actions-title"
	>
		<div class="border-b border-surface-300-700 p-6 max-[650px]:p-5">
			<h2 id="actions-title" class="m-0 text-xl font-bold text-surface-900-100">
				Actions prioritaires
			</h2>
			<p class="mt-1 mb-0 text-sm text-surface-700-300">
				Les décisions qui peuvent attendre le moins longtemps.
			</p>
		</div>
		<div class="divide-y divide-surface-200-800">
			{#each actions as action (action.title)}
				{@const ActionIcon = action.icon}
				<a
					href={action.href}
					class="group flex items-center gap-4 px-6 py-4 transition hover:bg-surface-100-900 max-[650px]:items-start max-[650px]:px-5"
				>
					<div class="grid size-9 shrink-0 place-items-center rounded-full {softTone(action.tone)}">
						<ActionIcon size={18} />
					</div>
					<div class="min-w-0 flex-1">
						<strong class="block text-sm text-surface-900-100">{action.title}</strong><span
							class="mt-1 block text-xs text-surface-700-300">{action.detail}</span
						>
					</div>
					<Badge variant="tonal" tone={action.tone}>{action.module}</Badge>
					<ArrowRight
						class="text-surface-500-400 shrink-0 transition group-hover:translate-x-1"
						size={17}
					/>
				</a>
			{/each}
		</div>
	</section>

	<section
		class="mb-6 card border border-surface-300-700 bg-surface-50-950 shadow-sm"
		aria-labelledby="overview-title"
	>
		<div class="border-b border-surface-300-700 p-6 max-[650px]:p-5">
			<h2 id="overview-title" class="m-0 text-xl font-bold text-surface-900-100">Vue d’ensemble</h2>
			<p class="mt-1 mb-0 text-sm text-surface-700-300">
				Un résumé utile de chaque activité, avec le prochain point d’attention.
			</p>
		</div>
		<div class="grid grid-cols-3 gap-3 p-3 max-[1000px]:grid-cols-2 max-[650px]:grid-cols-1">
			{#each summaries as summary (summary.name)}
				{@const SummaryIcon = summary.icon}
				<article
					class="rounded-base border border-surface-300-700 bg-surface-50-950 p-4 transition hover:shadow-sm"
				>
					<div class="mb-4 flex items-center justify-between gap-3">
						<div class="flex items-center gap-2 text-sm font-semibold text-surface-900-100">
							<SummaryIcon size={17} />{summary.name}
						</div>
						<Badge variant="tonal" tone={summary.tone}
							>{summary.tone === Tone.Success ? 'À jour' : 'À surveiller'}</Badge
						>
					</div>
					<strong class="block text-lg text-surface-900-100">{summary.value}</strong>
					<p class="mt-1 mb-4 min-h-8 text-xs text-surface-700-300">{summary.detail}</p>
					<a
						href={summary.href}
						class="inline-flex items-center gap-1 text-xs font-semibold text-primary-700-300 transition hover:text-tertiary-700-300"
						>{summary.action}<ArrowRight size={14} /></a
					>
				</article>
			{/each}
		</div>
	</section>

	<section class="grid grid-cols-[1.3fr_1fr] gap-4 max-[850px]:grid-cols-1">
		<div
			class="card border border-surface-300-700 bg-surface-50-950 shadow-sm"
			aria-labelledby="workshop-title"
		>
			<div class="border-b border-surface-300-700 p-6 max-[650px]:p-5">
				<h2 id="workshop-title" class="m-0 text-xl font-bold text-surface-900-100">Atelier</h2>
				<p class="mt-1 mb-0 text-sm text-surface-700-300">
					État actuel des machines et prochaines interventions.
				</p>
			</div>
			<div class="divide-y divide-surface-200-800">
				{#each data.workshop as machine (machine.name)}
					<div class="flex items-center gap-4 px-6 py-4 max-[650px]:px-5">
						<div class="size-2.5 shrink-0 rounded-full {dotTone(machine.tone)}"></div>
						<div class="min-w-0 flex-1">
							<strong class="block text-sm text-surface-900-100">{machine.name}</strong><span
								class="mt-1 block text-xs text-surface-700-300">{machine.detail}</span
							>
						</div>
						<Badge variant="tonal" tone={machine.tone}>{machine.state}</Badge>
					</div>
				{/each}
			</div>
		</div>
		<div
			class="card border border-surface-300-700 bg-surface-50-950 p-6 shadow-sm max-[650px]:p-5"
			aria-labelledby="cash-title"
		>
			<div class="mb-6 flex items-start justify-between gap-3">
				<div>
					<h2 id="cash-title" class="m-0 text-xl font-bold text-surface-900-100">
						Trésorerie prudente
					</h2>
					<p class="mt-1 mb-0 text-sm text-surface-700-300">
						Après provisions et factures à venir.
					</p>
				</div>
				<Truck class="text-tertiary-600-400" size={21} />
			</div>
			<strong class="block text-3xl font-bold text-surface-900-100">9 629 €</strong>
			<div class="mt-5 grid gap-3 border-t border-surface-300-700 pt-4 text-sm">
				<div class="flex justify-between gap-3">
					<span class="text-surface-700-300">À encaisser</span><strong class="text-surface-900-100"
						>1 240 €</strong
					>
				</div>
				<div class="flex justify-between gap-3">
					<span class="text-surface-700-300">Provisions</span><strong class="text-surface-900-100"
						>2 180 €</strong
					>
				</div>
				<a
					href="/finance"
					class="mt-2 inline-flex items-center gap-1 text-xs font-semibold text-primary-700-300"
					>Ouvrir la finance<ArrowRight size={14} /></a
				>
			</div>
		</div>
	</section>
</div>
