<script lang="ts">
	import ChevronLeft from '@lucide/svelte/icons/chevron-left';
	import ChevronRight from '@lucide/svelte/icons/chevron-right';
	import type { Snippet } from 'svelte';

	let {
		children,
		loading = false,
		loadingRows = 5,
		loadingColumns = 6,
		responsiveCards = false,
		page,
		pageSize,
		totalItems,
		totalPages,
		onPageChange,
		class: tableClass = ''
	}: {
		children?: Snippet;
		loading?: boolean;
		loadingRows?: number;
		loadingColumns?: number;
		responsiveCards?: boolean;
		page?: number;
		pageSize?: number;
		totalItems?: number;
		totalPages?: number;
		onPageChange?: (page: number) => void | Promise<void>;
		class?: string;
	} = $props();

	let paginated = $derived(
		page !== undefined &&
			pageSize !== undefined &&
			totalItems !== undefined &&
			totalPages !== undefined &&
			onPageChange !== undefined
	);
	let firstItem = $derived(totalItems === 0 ? 0 : ((page ?? 1) - 1) * (pageSize ?? 0) + 1);
	let lastItem = $derived(Math.min((page ?? 1) * (pageSize ?? 0), totalItems ?? 0));
</script>

<div class="table-component" aria-busy={loading}>
	<div class="table-wrap" class:responsive-cards={responsiveCards}>
		<table class="table text-sm {tableClass}">
			{#if loading}
				<tbody>
					{#each Array.from({ length: loadingRows }, (_, index) => index) as row (row)}
						<tr aria-hidden="true">
							{#each Array.from({ length: loadingColumns }, (_, index) => index) as column (column)}
								<td><span class="block h-3 placeholder w-full animate-pulse"></span></td>
							{/each}
						</tr>
					{/each}
				</tbody>
			{:else if children}
				{@render children()}
			{/if}
		</table>
	</div>
	{#if paginated}
		<nav
			class="flex items-center justify-between gap-4 border-t border-surface-300-700 px-4 py-3"
			aria-label="Pagination du tableau"
		>
			<p class="m-0 text-sm text-surface-700-300">
				{firstItem}–{lastItem} sur {totalItems}
			</p>
			<div class="flex items-center gap-2">
				<button
					class="btn-icon preset-tonal-surface"
					type="button"
					disabled={loading || page === 1}
					aria-label="Page précédente"
					onclick={() => onPageChange?.((page ?? 1) - 1)}
				>
					<ChevronLeft size={16} />
				</button>
				<span class="min-w-20 text-center text-sm text-surface-700-300">
					Page {page} sur {Math.max(totalPages ?? 0, 1)}
				</span>
				<button
					class="btn-icon preset-tonal-surface"
					type="button"
					disabled={loading || page === totalPages || totalPages === 0}
					aria-label="Page suivante"
					onclick={() => onPageChange?.((page ?? 1) + 1)}
				>
					<ChevronRight size={16} />
				</button>
			</div>
		</nav>
	{/if}
</div>

<style>
	.table-wrap {
		width: 100%;
		overflow: auto;
	}
	.table-component {
		--text-scaling: 1;
	}
	.table {
		min-width: 760px;
		font-variant-numeric: tabular-nums;
	}
	:global(th) {
		font-size: 0.76rem;
		letter-spacing: 0.06em;
		text-transform: uppercase;
	}
	:global(td strong),
	:global(td small) {
		display: block;
	}
	:global(td strong) {
		color: var(--color-surface-900-100);
	}
	:global(td small) {
		margin-top: 2px;
		color: var(--color-surface-700-300);
		font-size: 0.85rem;
	}
	@media (max-width: 700px) {
		.responsive-cards {
			overflow: visible;
		}
		.responsive-cards .table {
			display: block;
			min-width: 0;
		}
		.responsive-cards > .table > :global(colgroup) {
			display: none;
		}
		.responsive-cards > .table > :global(thead) {
			position: absolute;
			width: 1px;
			height: 1px;
			overflow: hidden;
			clip: rect(0 0 0 0);
			white-space: nowrap;
		}
		.responsive-cards > .table > :global(tbody),
		.responsive-cards > .table > :global(tbody > tr) {
			display: grid;
			gap: 0;
		}
		.responsive-cards > .table > :global(tbody) {
			gap: 12px;
			padding: 12px;
		}
		.responsive-cards > .table > :global(tbody > tr) {
			overflow: hidden;
			border: 1px solid var(--color-surface-300-700);
			border-radius: var(--radius-base);
			background: var(--color-surface-50-950);
			box-shadow: var(--shadow-sm);
		}
		.responsive-cards > .table > :global(tbody > tr > td) {
			display: grid;
			grid-template-columns: minmax(7.5rem, 38%) minmax(0, 1fr);
			align-items: center;
			gap: 12px;
			min-height: 44px;
			padding: 9px 12px;
			border-bottom: 1px solid var(--color-surface-200-800);
			text-align: left;
		}
		.responsive-cards > .table > :global(tbody > tr > td:last-child) {
			border-bottom: 0;
		}
		.responsive-cards > .table > :global(tbody > tr > td::before) {
			content: attr(data-label);
			font-size: 0.76rem;
			font-weight: 700;
			letter-spacing: 0.045em;
			text-transform: uppercase;
			color: var(--color-surface-700-300);
		}
		.responsive-cards > .table > :global(tbody > tr > td[data-label='']) {
			grid-template-columns: 1fr;
		}
		.responsive-cards > .table > :global(tbody > tr > td[data-label='']::before) {
			display: none;
		}
		.responsive-cards > .table > :global(tbody > tr > .mobile-card-hidden) {
			display: none;
		}
		.responsive-cards > .table > :global(tbody > tr > .mobile-card-actions) {
			display: flex;
			justify-content: flex-end;
			gap: 4px;
		}
	}
</style>
