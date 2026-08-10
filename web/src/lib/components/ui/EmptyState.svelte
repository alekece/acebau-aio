<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		title,
		description,
		icon,
		actions,
		compact = false
	}: {
		title: string;
		description?: string;
		icon?: Snippet;
		actions?: Snippet;
		compact?: boolean;
	} = $props();
</script>

<div
	class="grid place-content-center justify-items-center text-center {compact
		? 'min-h-52 gap-2 px-5 py-9'
		: 'min-h-[360px] gap-3 px-6 py-12'}"
>
	{#if icon}
		<div
			class="grid size-14 place-items-center rounded-container bg-tertiary-100-900 text-tertiary-600-400"
		>
			{@render icon()}
		</div>
	{/if}
	<h2 class="m-0 text-xl font-bold text-surface-900-100">{title}</h2>
	{#if description}<p class="m-0 max-w-lg text-sm text-surface-700-300">{description}</p>{/if}
	{#if actions}<div class="mt-2 flex flex-wrap justify-center gap-2">{@render actions()}</div>{/if}
</div>
