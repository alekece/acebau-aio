<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		children,
		class: className = ''
	}: {
		children: Snippet;
		class?: string;
	} = $props();
</script>

<tr class="bg-tertiary-50-950/40 {className}" data-table-draft-row>
	{@render children()}
</tr>
