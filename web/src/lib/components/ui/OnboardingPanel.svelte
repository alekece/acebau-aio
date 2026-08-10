<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		title,
		steps,
		currentStep = 0,
		children
	}: {
		title: string;
		steps: string[];
		currentStep?: number;
		children: Snippet;
	} = $props();
</script>

<section
	class="rounded-container border border-surface-300-700 bg-surface-50-950 p-5 shadow-sm sm:p-8"
>
	<h2 class="m-0 text-2xl font-bold text-surface-900-100">{title}</h2>
	<ol
		class="my-7 grid gap-2 sm:grid-cols-[repeat(var(--step-count),minmax(0,1fr))]"
		style={`--step-count:${steps.length}`}
	>
		{#each steps as step, index (step)}
			<li
				class="flex min-w-0 items-center gap-2 text-sm {index === currentStep
					? 'font-semibold text-tertiary-700-300'
					: index < currentStep
						? 'text-success-700-300'
						: 'text-surface-600-400'}"
				aria-current={index === currentStep ? 'step' : undefined}
			>
				<span
					class="grid size-7 shrink-0 place-items-center rounded-full {index === currentStep
						? 'bg-tertiary-500 text-tertiary-contrast-500'
						: index < currentStep
							? 'bg-success-500 text-success-contrast-500'
							: 'bg-surface-200-800 text-surface-700-300'}">{index + 1}</span
				>
				<span>{step}</span>
			</li>
		{/each}
	</ol>
	{@render children()}
</section>
