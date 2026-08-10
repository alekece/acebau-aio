<script lang="ts">
	import type { Snippet } from 'svelte';

	let { children, class: className = '' }: { children: Snippet; class?: string } = $props();
</script>

<div
	class="mx-auto w-full max-w-[1240px] px-5 py-7 pb-16 min-[851px]:px-12 min-[851px]:py-10 {className}"
>
	{@render children()}
</div>
