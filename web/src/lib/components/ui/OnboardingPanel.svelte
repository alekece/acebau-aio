<script lang="ts">
	import { Steps } from '@skeletonlabs/skeleton-svelte';
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
	<Steps step={currentStep} count={steps.length} linear class="my-7 w-full">
		<Steps.List>
			{#each steps as step, index (step)}
				<Steps.Item {index} class="min-w-0">
					<Steps.Trigger class="cursor-default text-sm text-surface-600-400">
						<Steps.Indicator
							class="data-[complete]:border-tertiary-500 data-[complete]:bg-tertiary-500 data-[complete]:text-tertiary-contrast-500 data-[current]:border-tertiary-500"
							>{index + 1}</Steps.Indicator
						>
						<span>{step}</span>
					</Steps.Trigger>
					{#if index < steps.length - 1}
						<Steps.Separator class="data-[complete]:border-tertiary-500" />
					{/if}
				</Steps.Item>
			{/each}
		</Steps.List>
	</Steps>
	{@render children()}
</section>
