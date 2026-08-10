<script lang="ts">
	import '../app.css';
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import Footer from '$lib/components/Footer.svelte';
	import { AppBar } from '@skeletonlabs/skeleton-svelte';
	import ChartNoAxesCombined from '@lucide/svelte/icons/chart-no-axes-combined';
	import Settings from '@lucide/svelte/icons/settings';
	import LayoutDashboard from '@lucide/svelte/icons/layout-dashboard';
	import Factory from '@lucide/svelte/icons/factory';
	import { Navigation } from '@skeletonlabs/skeleton-svelte';
	import LightSwitch from '$lib/components/LightSwitch.svelte';
	let { children }: LayoutProps = $props();
	let activeItem = $derived(page.url.pathname);
	let isExpanded = $state(false);
</script>

<div class="flex min-h-screen flex-col">
	<AppBar background="bg-primary-900 text-primary-contrast-900">
		<p class="preset-typo-title">Acebau</p>
		{#snippet trail()}
			<LightSwitch />
		{/snippet}
	</AppBar>
	<div class="flex flex-1">
		<div class="full flex">
			<Navigation.Rail expanded={isExpanded} background="bg-primary-800 text-primary-contrast-800">
				{#snippet header()}
					<Navigation.Tile labelExpanded="Dashboard" href="/">
						<LayoutDashboard size="24" />
					</Navigation.Tile>
					<Navigation.Tile labelExpanded="Printing Environment" href="/printing_environments">
						<Factory size="24" />
					</Navigation.Tile>
					<Navigation.Tile labelExpanded="Analytics" href="/analytics">
						<ChartNoAxesCombined size="24" />
					</Navigation.Tile>
				{/snippet}
				{#snippet tiles()}{/snippet}
				{#snippet footer()}
					<Navigation.Tile labelExpanded="Settings" href="/settings" title="Settings"
						><Settings size="24" /></Navigation.Tile
					>
				{/snippet}
			</Navigation.Rail>
		</div>

		<main class="w-full p-8">
			{@render children()}
		</main>
	</div>
	<Footer />
</div>
