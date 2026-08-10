<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		title,
		description,
		toolbar,
		children,
		id,
		class: className = ''
	}: {
		title?: string;
		description?: string;
		toolbar?: Snippet;
		children: Snippet;
		id?: string;
		class?: string;
	} = $props();
</script>

<section
	{id}
	class="overflow-hidden rounded-container border border-surface-300-700 bg-surface-50-950 shadow-sm {className}"
>
	{#if title || description || toolbar}
		<div
			class="flex flex-wrap items-center justify-between gap-4 border-b border-surface-300-700 p-5"
		>
			{#if title || description}
				<div>
					{#if title}<h2 class="m-0 text-xl font-bold text-surface-900-100">{title}</h2>{/if}
					{#if description}<p class="mt-1 mb-0 text-sm text-surface-700-300">{description}</p>{/if}
				</div>
			{/if}
			{#if toolbar}<div class="flex flex-wrap items-center gap-2">{@render toolbar()}</div>{/if}
		</div>
	{/if}
	{@render children()}
</section>
