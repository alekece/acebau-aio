<script lang="ts">
	let {
		label = '',
		value = '',
		detail = '',
		loading = false
	}: { label?: string; value?: string | number; detail?: string; loading?: boolean } = $props();
</script>

<article
	class="kpi grid gap-2 card border border-surface-300-700 bg-surface-50-950 p-5 shadow-sm"
	aria-busy={loading}
>
	{#if loading}
		<span class="h-3 placeholder w-2/3 animate-pulse"></span>
		<span class="h-6 placeholder w-1/3 animate-pulse"></span>
		<span class="h-3 placeholder w-4/5 animate-pulse"></span>
	{:else}
		<span class="text-xs font-semibold tracking-wide text-surface-700-300 uppercase">{label}</span>
		<strong class="text-lg leading-none text-surface-900-100">{value}</strong>
		{#if detail}<span class="text-xs text-surface-700-300">{detail}</span>{/if}
	{/if}
</article>

<style>
	.kpi {
		--text-scaling: 1;
	}
</style>
