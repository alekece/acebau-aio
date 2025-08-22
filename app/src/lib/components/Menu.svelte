<script lang="ts">
	let { items } = $props();
</script>


<nav class="flex items-center gap-4">
	{#each items as item}
		<button type="button" class="btn preset-filled">{item.title}</button>
	{/each}
</nav>
