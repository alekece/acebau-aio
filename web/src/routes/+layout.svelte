<script lang="ts">
	import '../app.css';
	import type { LayoutProps } from './$types';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { AppBar } from '@skeletonlabs/skeleton-svelte';
	import LightSwitch from '$lib/components/ui/LightSwitch.svelte';
	import Bell from '@lucide/svelte/icons/bell';
	import Menu from '@lucide/svelte/icons/menu';
	import { setContext } from 'svelte';
	import { METRIC_DEFAULTS_CONTEXT } from '$lib/settings/metric-defaults';
	let { children, data }: LayoutProps = $props();
	let mobileNavigationOpen = $state(false);
	setContext(METRIC_DEFAULTS_CONTEXT, () => data.metricDefaults);
</script>

<svelte:window
	onkeydown={(event) => {
		if (event.key === 'Escape') mobileNavigationOpen = false;
	}}
/>

<div class="flex h-screen overflow-hidden">
	{#if mobileNavigationOpen}
		<button
			type="button"
			class="fixed inset-0 z-40 bg-surface-950/55 backdrop-blur-[1px] min-[851px]:hidden"
			aria-label="Close navigation"
			onclick={() => (mobileNavigationOpen = false)}
		></button>
	{/if}
	<Sidebar open={mobileNavigationOpen} onClose={() => (mobileNavigationOpen = false)} />

	<div class="flex min-h-0 min-w-0 flex-1 flex-col">
		<header class="shrink-0">
			<AppBar class="bg-surface-50-950 text-surface-900-100">
				<AppBar.Toolbar
					class="flex w-full items-center justify-between px-4 py-3 min-[851px]:justify-end min-[851px]:px-6"
				>
					<AppBar.Lead class="min-[851px]:hidden">
						<button
							type="button"
							class="flex size-11 items-center justify-center rounded-base border border-surface-300-700 text-surface-900-100 hover:bg-surface-200-800"
							aria-label="Open navigation"
							aria-expanded={mobileNavigationOpen}
							onclick={() => (mobileNavigationOpen = true)}
						>
							<Menu size={21} />
						</button>
					</AppBar.Lead>
					<AppBar.Trail class="ml-auto flex items-center">
						<div class="flex items-center gap-3">
							<button
								class="relative flex size-9 items-center justify-center rounded-full border border-surface-300-700 text-surface-900-100 transition hover:bg-surface-200-800"
								type="button"
								aria-label="Notifications"
								title="Notifications"
							>
								<Bell size="17" />
								<span
									class="absolute -top-1 -right-1 flex size-4 items-center justify-center rounded-full bg-error-500 text-[0.65rem] font-bold text-error-contrast-500"
									>3</span
								>
							</button>
							<LightSwitch />
						</div>
					</AppBar.Trail>
				</AppBar.Toolbar>
			</AppBar>
		</header>

		<main class="min-w-0 flex-1 overflow-y-auto bg-surface-100 dark:bg-surface-700">
			{@render children()}
		</main>
	</div>
</div>
