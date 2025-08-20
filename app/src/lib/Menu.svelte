<script lang="module">
</script>

<script lang="ts">
	export MenuItem = {
		href: string;
		title: string;
	};

	let { items } = $props();
</script>


<nav>
	{#each items as item}
		<a href={item.slug}>{item.title}</a>
	{/each}
</nav>
