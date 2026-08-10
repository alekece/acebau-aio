<script lang="ts">
	import TrendingUp from '@lucide/svelte/icons/trending-up';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Kpi from '$lib/components/ui/Kpi.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import ModuleHeader from '$lib/components/ui/ModuleHeader.svelte';
	import { Tone } from '$lib/types';
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();
	let period = $state('Cette année');
</script>

<svelte:head><title>Données — Acebau</title></svelte:head>
<div class="mx-auto max-w-[1240px] px-12 py-10 pb-16 max-[850px]:px-5 max-[850px]:py-7">
	<ModuleHeader
		title="Données"
		description="Comprenez ce qui se vend, ce qui crée de la marge et ce qui mérite d’évoluer."
	/>
	<div class="mb-6 flex flex-wrap gap-3">
		<Input as="select" bind:value={period} aria-label="Période"
			><option>Cette année</option><option>30 derniers jours</option><option
				>Année précédente</option
			></Input
		><Input as="select" aria-label="Canal"
			><option>Tous les canaux</option><option>Revendeur</option><option>Site</option><option
				>Direct</option
			></Input
		><Button variant="outlined" tone="surface">Réinitialiser</Button>
	</div>
	<section class="mb-6 grid grid-cols-5 gap-3.5 max-[1100px]:grid-cols-3 max-[650px]:grid-cols-1">
		<Kpi label="Objets vendus" value="91" detail="+18 % versus période précédente" /><Kpi
			label="CA produits"
			value="11 300 €"
			detail="HT, toutes activités"
		/><Kpi label="Marge brute" value="56 %" detail="+4 points" /><Kpi
			label="Premier contributeur"
			value="ONDRA"
			detail="52 % du CA produits"
		/><Kpi label="Sous-performance" value="Support mural" detail="Marge à revoir" />
	</section>
	<div class="grid gap-6 lg:grid-cols-[1.1fr_0.9fr]">
		<section class="card border border-surface-300-700 bg-surface-50-950 p-6 shadow-sm">
			<div class="flex items-start justify-between">
				<div>
					<h2 class="m-0 text-lg font-bold text-surface-900-100">Ventes mensuelles</h2>
					<p class="mt-1 text-sm text-surface-700-300">Chiffre d’affaires produit HT</p>
				</div>
				<TrendingUp class="text-success-500" size={22} />
			</div>
			<div class="mt-7 flex h-44 items-end gap-3 border-b border-surface-300-700 px-2">
				{#each [['Jan', '42 %'], ['Fév', '58 %'], ['Mar', '48 %'], ['Avr', '72 %'], ['Mai', '64 %'], ['Juin', '82 %'], ['Juil', '76 %'], ['Août', '92 %']] as bar (bar[0])}<div
						class="flex flex-1 flex-col items-center gap-2"
					>
						<div
							class="w-full rounded-t bg-tertiary-500/80 transition hover:bg-tertiary-500"
							style={`height:${bar[1]}`}
						></div>
						<span class="text-xs text-surface-700-300">{bar[0]}</span>
					</div>{/each}
			</div>
		</section>
		<section class="card border border-surface-300-700 bg-surface-50-950 p-6 shadow-sm">
			<h2 class="m-0 text-lg font-bold text-surface-900-100">Canaux</h2>
			<p class="mt-1 text-sm text-surface-700-300">CA et marge par canal</p>
			{#each [['Revendeur', '5 240 €', '48 %', Tone.Secondary], ['Site', '4 180 €', '62 %', Tone.Tertiary], ['Direct', '1 880 €', '71 %', Tone.Success]] as channel (channel[0])}<div
					class="mt-5 flex items-center justify-between"
				>
					<div>
						<strong class="block text-sm text-surface-900-100">{channel[0]}</strong><span
							class="text-xs text-surface-700-300">{channel[1]}</span
						>
					</div>
					<Badge variant="tonal" tone={channel[3]}>{channel[2]}</Badge>
				</div>{/each}
		</section>
	</div>
	<section
		class="mt-6 overflow-hidden card border border-surface-300-700 bg-surface-50-950 shadow-sm"
	>
		<div class="flex items-center justify-between border-b border-surface-300-700 p-5">
			<div>
				<h2 class="m-0 text-lg font-bold text-surface-900-100">Performance produits</h2>
				<p class="mt-1 mb-0 text-sm text-surface-700-300">
					Unités, CA et marge sur la période choisie.
				</p>
			</div>
		</div>
		<div class="grid grid-cols-4 gap-4 p-5 max-[700px]:grid-cols-1">
			{#each data.products as product (product[0])}<div
					class="rounded-base border border-surface-300-700 p-4"
				>
					<div class="flex items-center justify-between">
						<strong class="text-surface-900-100">{product[0]}</strong><Badge
							variant="tonal"
							tone={product[4]}>{product[3]}</Badge
						>
					</div>
					<div class="mt-5 text-2xl font-bold text-surface-900-100">{product[2]}</div>
					<div class="mt-1 text-xs text-surface-700-300">{product[1]} unités vendues</div>
				</div>{/each}
		</div>
	</section>
</div>
