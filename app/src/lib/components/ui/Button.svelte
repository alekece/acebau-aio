<script lang="ts">
	import { presetGroups, Variant } from '$lib/types';
	import { consumeExclusiveGroups } from '$lib/utils/exclusive';

	let props = $props();

	let {
		normalizedProps: { size, variant, tone },
		rest: { disabled, class: ClassValue, children, ...rest }
	} = $derived.by(() => consumeExclusiveGroups('Button', props, presetGroups));

	let preset = $derived(
		`preset-${variant}${tone ? (variant == Variant.Tonal ? `-${tone}` : `-${tone}-500`) : ''}`
	);
</script>

<button type="button" class="btn btn-{size} {preset} {rest.class}" {disabled} {...rest}>
	{@render children()}
</button>
