<script lang="ts">
	import { Variant, presetGroups } from '$lib/types';
	import { consumeExclusiveGroups } from '$lib/utils/exclusive';

	let props = $props();

	let {
		normalizedProps: { variant, tone },
		rest: { disabled, class: ClassValue, icon, children, ...rest }
	} = $derived.by(() => consumeExclusiveGroups('Button', props, presetGroups));

	let preset = $derived(
		`preset-${variant}${tone ? (variant == Variant.Tonal ? `-${tone}` : `-${tone}-500`) : ''}`
	);
</script>

<span class="badge {preset} {rest.class}">
	{#if icon}
		{@render icon()}
		<span>
			{@render children()}
		</span>
	{:else}
		{@render children()}
	{/if}
</span>
