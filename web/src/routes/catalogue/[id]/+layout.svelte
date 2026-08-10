<script lang="ts">
	import ArrowLeft from '@lucide/svelte/icons/arrow-left';
	import Badge from '$lib/components/ui/Badge.svelte';
	import DataSourceNotice from '$lib/components/ui/DataSourceNotice.svelte';
	import PageShell from '$lib/components/ui/PageShell.svelte';
	import type { LayoutProps } from './$types';

	let { data, children }: LayoutProps = $props();
</script>

<PageShell class="max-w-[1120px]">
	<a
		href="/catalogue"
		class="mb-5 inline-flex min-h-11 items-center gap-1.5 text-sm font-medium text-surface-700-300 hover:text-surface-900-100"
	>
		<ArrowLeft size={16} />Catalogue
	</a>
	<DataSourceNotice visible={data.usingFallback} message={data.loadError} />
	<header class="mb-5 flex flex-wrap items-start justify-between gap-4">
		<div>
			<p class="m-0 text-sm font-semibold text-tertiary-700-300">
				{data.product.collection} · {data.product.category}
			</p>
			<h1 class="mt-1 mb-2 text-3xl font-bold text-surface-900-100">{data.product.name}</h1>
			{#if data.product.shortDescription}<p class="m-0 max-w-2xl text-surface-700-300">
					{data.product.shortDescription}
				</p>{/if}
		</div>
		<Badge variant="tonal" tone={data.product.status === 'active' ? 'success' : 'warning'}>
			{data.product.status === 'active' ? 'Actif' : 'Brouillon'}
		</Badge>
	</header>
	<nav
		class="sticky top-0 z-20 mb-6 flex gap-1 overflow-x-auto border-y border-surface-300-700 bg-surface-100/95 py-2 backdrop-blur dark:bg-surface-700/95"
		aria-label="Sections du produit"
	>
		<a
			class="rounded-base px-3 py-2 text-sm font-medium hover:bg-surface-200-800"
			href={`/catalogue/${data.product.id}`}>Vue d’ensemble</a
		>
		<a
			class="rounded-base px-3 py-2 text-sm font-medium hover:bg-surface-200-800"
			href={`/catalogue/${data.product.id}#variantes`}>Variantes</a
		>
	</nav>
	{@render children()}
</PageShell>
