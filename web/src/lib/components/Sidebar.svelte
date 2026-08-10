<script lang="ts">
	import { page } from '$app/state';
	import { Navigation as SkeletonNavigation } from '@skeletonlabs/skeleton-svelte';
	import LayoutDashboard from '@lucide/svelte/icons/layout-dashboard';
	import BarChart3 from '@lucide/svelte/icons/bar-chart-3';
	import Box from '@lucide/svelte/icons/box';
	import ClipboardList from '@lucide/svelte/icons/clipboard-list';
	import Factory from '@lucide/svelte/icons/factory';
	import FileText from '@lucide/svelte/icons/file-text';
	import Landmark from '@lucide/svelte/icons/landmark';
	import Package from '@lucide/svelte/icons/package';
	import Printer from '@lucide/svelte/icons/printer';
	import Settings from '@lucide/svelte/icons/settings';
	import Users from '@lucide/svelte/icons/users';
	import X from '@lucide/svelte/icons/x';
	import { onMount } from 'svelte';

	let { open = false, onClose }: { open?: boolean; onClose?: () => void } = $props();
	let currentPath = $derived(page.url.pathname);
	let previousPath = page.url.pathname;
	let mobileViewport = $state(false);

	onMount(() => {
		const query = window.matchMedia('(max-width: 850px)');
		const update = () => (mobileViewport = query.matches);
		update();
		query.addEventListener('change', update);
		return () => query.removeEventListener('change', update);
	});

	$effect(() => {
		const nextPath = page.url.pathname;
		if (nextPath !== previousPath) {
			previousPath = nextPath;
			onClose?.();
		}
	});
</script>

<aside
	class="fixed inset-y-0 left-0 z-50 h-screen shrink-0 bg-primary-800 text-primary-contrast-800 shadow-xl transition-transform duration-200 min-[851px]:relative min-[851px]:z-auto min-[851px]:translate-x-0 min-[851px]:shadow-none {open
		? 'translate-x-0'
		: '-translate-x-full'}"
	aria-label="Primary navigation"
	aria-hidden={mobileViewport && !open}
	inert={mobileViewport && !open}
>
	<SkeletonNavigation
		layout="sidebar"
		class="flex h-full w-56 flex-col bg-primary-800 text-primary-contrast-800"
	>
		<SkeletonNavigation.Header class="mx-3 border-b border-primary-contrast-800/15 px-0 pt-3 pb-4">
			<div class="flex items-center justify-between gap-3">
				<div class="flex items-center gap-3">
					<div
						class="flex size-9 shrink-0 items-center justify-center rounded-full border border-primary-contrast-800/30 bg-primary-contrast-800/10"
						aria-label="Logo Acebau"
					></div>
					<span class="text-base font-semibold tracking-tight text-primary-contrast-800"
						>Acebau</span
					>
				</div>
				<button
					type="button"
					class="grid size-11 place-items-center rounded-base hover:bg-primary-contrast-800/10 min-[851px]:hidden"
					aria-label="Fermer la navigation"
					onclick={onClose}><X size={20} /></button
				>
			</div>
		</SkeletonNavigation.Header>
		<SkeletonNavigation.Content class="min-h-0 flex-1 px-3 py-4">
			<div class="flex flex-col gap-5">
				<SkeletonNavigation.Group>
					<SkeletonNavigation.Label
						class="px-2 pb-1 text-[0.65rem] font-bold tracking-wider text-primary-contrast-800/70 uppercase"
						>Pilotage</SkeletonNavigation.Label
					>
					<SkeletonNavigation.Menu>
						<SkeletonNavigation.TriggerAnchor
							href="/"
							class="flex items-center gap-3 rounded-base px-3 py-2 text-sm transition {currentPath ===
							'/'
								? 'bg-primary-contrast-800/15 font-semibold'
								: 'hover:bg-primary-contrast-800/10'}"
							aria-current={currentPath === '/' ? 'page' : undefined}
						>
							<LayoutDashboard size="18" />Tableau de bord
						</SkeletonNavigation.TriggerAnchor>
					</SkeletonNavigation.Menu>
				</SkeletonNavigation.Group>
				<SkeletonNavigation.Group>
					<SkeletonNavigation.Label
						class="px-2 pb-1 text-[0.65rem] font-bold tracking-wider text-primary-contrast-800/70 uppercase"
						>Atelier</SkeletonNavigation.Label
					>
					<SkeletonNavigation.Menu>
						<SkeletonNavigation.TriggerAnchor
							href="/production"
							class="flex items-center gap-3 rounded-base px-3 py-2 text-sm transition {currentPath.startsWith(
								'/production'
							)
								? 'bg-primary-contrast-800/15 font-semibold'
								: 'hover:bg-primary-contrast-800/10'}"
							aria-current={currentPath.startsWith('/production') ? 'page' : undefined}
						>
							<Factory size="18" />Production
						</SkeletonNavigation.TriggerAnchor>
						<SkeletonNavigation.TriggerAnchor
							href="/machines"
							class="flex items-center gap-3 rounded-base px-3 py-2 text-sm transition {currentPath.startsWith(
								'/machines'
							)
								? 'bg-primary-contrast-800/15 font-semibold'
								: 'hover:bg-primary-contrast-800/10'}"
							aria-current={currentPath.startsWith('/machines') ? 'page' : undefined}
						>
							<Printer size="18" />Machines
						</SkeletonNavigation.TriggerAnchor>
						<SkeletonNavigation.TriggerAnchor
							href="/inventory"
							class="flex items-center gap-3 rounded-base px-3 py-2 text-sm transition {currentPath.startsWith(
								'/inventory'
							)
								? 'bg-primary-contrast-800/15 font-semibold'
								: 'hover:bg-primary-contrast-800/10'}"
							aria-current={currentPath.startsWith('/inventory') ? 'page' : undefined}
						>
							<Package size="18" />Inventaire
						</SkeletonNavigation.TriggerAnchor>
					</SkeletonNavigation.Menu>
				</SkeletonNavigation.Group>
				<SkeletonNavigation.Group>
					<SkeletonNavigation.Label
						class="px-2 pb-1 text-[0.65rem] font-bold tracking-wider text-primary-contrast-800/70 uppercase"
						>Offre</SkeletonNavigation.Label
					>
					<SkeletonNavigation.Menu>
						<SkeletonNavigation.TriggerAnchor
							href="/catalogue"
							class="flex items-center gap-3 rounded-base px-3 py-2 text-sm transition {currentPath.startsWith(
								'/catalogue'
							)
								? 'bg-primary-contrast-800/15 font-semibold'
								: 'hover:bg-primary-contrast-800/10'}"
							aria-current={currentPath.startsWith('/catalogue') ? 'page' : undefined}
						>
							<Box size="18" />Catalogue
						</SkeletonNavigation.TriggerAnchor>
						<SkeletonNavigation.TriggerAnchor
							href="/pieces"
							class="flex items-center gap-3 rounded-base px-3 py-2 text-sm transition {currentPath.startsWith(
								'/pieces'
							)
								? 'bg-primary-contrast-800/15 font-semibold'
								: 'hover:bg-primary-contrast-800/10'}"
							aria-current={currentPath.startsWith('/pieces') ? 'page' : undefined}
						>
							<Package size="18" />Pièces
						</SkeletonNavigation.TriggerAnchor>
					</SkeletonNavigation.Menu>
				</SkeletonNavigation.Group>
				<SkeletonNavigation.Group>
					<SkeletonNavigation.Label
						class="px-2 pb-1 text-[0.65rem] font-bold tracking-wider text-primary-contrast-800/70 uppercase"
						>Ventes</SkeletonNavigation.Label
					>
					<SkeletonNavigation.Menu>
						<SkeletonNavigation.TriggerAnchor
							href="/orders"
							class="flex items-center gap-3 rounded-base px-3 py-2 text-sm transition {currentPath.startsWith(
								'/orders'
							)
								? 'bg-primary-contrast-800/15 font-semibold'
								: 'hover:bg-primary-contrast-800/10'}"
							aria-current={currentPath.startsWith('/orders') ? 'page' : undefined}
						>
							<ClipboardList size="18" />Commandes
						</SkeletonNavigation.TriggerAnchor>
						<SkeletonNavigation.TriggerAnchor
							href="/invoices"
							class="flex items-center gap-3 rounded-base px-3 py-2 text-sm transition {currentPath.startsWith(
								'/invoices'
							)
								? 'bg-primary-contrast-800/15 font-semibold'
								: 'hover:bg-primary-contrast-800/10'}"
							aria-current={currentPath.startsWith('/invoices') ? 'page' : undefined}
						>
							<FileText size="18" />Factures
						</SkeletonNavigation.TriggerAnchor>
						<SkeletonNavigation.TriggerAnchor
							href="/resellers"
							class="flex items-center gap-3 rounded-base px-3 py-2 text-sm transition {currentPath.startsWith(
								'/resellers'
							)
								? 'bg-primary-contrast-800/15 font-semibold'
								: 'hover:bg-primary-contrast-800/10'}"
							aria-current={currentPath.startsWith('/resellers') ? 'page' : undefined}
						>
							<Users size="18" />Revendeurs
						</SkeletonNavigation.TriggerAnchor>
					</SkeletonNavigation.Menu>
				</SkeletonNavigation.Group>
				<SkeletonNavigation.Group>
					<SkeletonNavigation.Label
						class="px-2 pb-1 text-[0.65rem] font-bold tracking-wider text-primary-contrast-800/70 uppercase"
						>Analyse</SkeletonNavigation.Label
					>
					<SkeletonNavigation.Menu>
						<SkeletonNavigation.TriggerAnchor
							href="/finance"
							class="flex items-center gap-3 rounded-base px-3 py-2 text-sm transition {currentPath.startsWith(
								'/finance'
							)
								? 'bg-primary-contrast-800/15 font-semibold'
								: 'hover:bg-primary-contrast-800/10'}"
							aria-current={currentPath.startsWith('/finance') ? 'page' : undefined}
						>
							<Landmark size="18" />Finance
						</SkeletonNavigation.TriggerAnchor>
						<SkeletonNavigation.TriggerAnchor
							href="/data"
							class="flex items-center gap-3 rounded-base px-3 py-2 text-sm transition {currentPath.startsWith(
								'/data'
							)
								? 'bg-primary-contrast-800/15 font-semibold'
								: 'hover:bg-primary-contrast-800/10'}"
							aria-current={currentPath.startsWith('/data') ? 'page' : undefined}
						>
							<BarChart3 size="18" />Données
						</SkeletonNavigation.TriggerAnchor>
					</SkeletonNavigation.Menu>
				</SkeletonNavigation.Group>
				<SkeletonNavigation.Group>
					<SkeletonNavigation.Label
						class="px-2 pb-1 text-[0.65rem] font-bold tracking-wider text-primary-contrast-800/70 uppercase"
						>Configuration</SkeletonNavigation.Label
					>
					<SkeletonNavigation.Menu>
						<SkeletonNavigation.TriggerAnchor
							href="/settings"
							class="flex items-center gap-3 rounded-base px-3 py-2 text-sm transition {currentPath.startsWith(
								'/settings'
							)
								? 'bg-primary-contrast-800/15 font-semibold'
								: 'hover:bg-primary-contrast-800/10'}"
							aria-current={currentPath.startsWith('/settings') ? 'page' : undefined}
						>
							<Settings size="18" />Paramètres
						</SkeletonNavigation.TriggerAnchor>
					</SkeletonNavigation.Menu>
				</SkeletonNavigation.Group>
			</div>
		</SkeletonNavigation.Content>
		<SkeletonNavigation.Footer
			class="mx-3 border-t border-primary-contrast-800/25 px-2 pt-3 pb-3 text-center text-xs font-medium text-primary-contrast-800/80"
		>
			v{__APP_VERSION__}
		</SkeletonNavigation.Footer>
	</SkeletonNavigation>
</aside>
