<script lang="ts">
	import ArrowLeft from '@lucide/svelte/icons/arrow-left';
	import Plus from '@lucide/svelte/icons/plus';
	import type { Snippet } from 'svelte';

	let {
		title,
		description,
		action,
		onAction,
		backHref,
		actions
	}: {
		title: string;
		description?: string;
		action?: string;
		onAction?: () => void;
		backHref?: string;
		actions?: Snippet;
	} = $props();
</script>

<header
	class="mb-7 flex items-end justify-between gap-6 max-[760px]:flex-col max-[760px]:items-start"
>
	<div>
		{#if backHref}<a
				href={backHref}
				class="mb-3 inline-flex items-center gap-1.5 text-sm text-surface-700-300 hover:text-surface-900-100"
				><ArrowLeft size={15} />Retour</a
			>{/if}
		<h1 class="m-0 text-3xl font-bold tracking-tight text-surface-900-100">{title}</h1>
		{#if description}<p class="mt-2 mb-0 max-w-2xl text-base text-surface-700-300">
				{description}
			</p>{/if}
	</div>
	{#if actions}
		<div class="flex flex-wrap items-center gap-2 max-[760px]:w-full">{@render actions()}</div>
	{:else if action}<button
			type="button"
			class="btn preset-filled-tertiary-500 btn-lg"
			onclick={() => onAction?.()}><Plus size={17} />{action}</button
		>{/if}
</header>
