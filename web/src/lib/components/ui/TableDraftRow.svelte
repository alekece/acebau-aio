<script lang="ts">
	import type { Snippet } from 'svelte';
	import { fly } from 'svelte/transition';

	let {
		children,
		class: className = '',
		animateRemoval = true
	}: {
		children: Snippet;
		class?: string;
		animateRemoval?: boolean;
	} = $props();
</script>

<tr
	class="bg-tertiary-50-950/40 {className}"
	data-table-draft-row
	in:fly={{ y: -8, duration: 180 }}
	out:fly={{ y: -8, duration: animateRemoval ? 180 : 0 }}
>
	{@render children()}
</tr>
