<script lang="ts">
	let { items } = $props();
</script>

<nav>
	{#each items as item}
		<a href={item.slug}>{item.title}</a>
	{/each}
</nav>
