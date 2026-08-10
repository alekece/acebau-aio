<script lang="ts">
    let {background = 'bg-surface-100 text-surface-contrast-100'} = $props();

	let date = new Date();
</script>

<footer class="flex w-full justify-center p-4 {background}">
	<p>© {date.getFullYear()} {__APP_NAME__} {__APP_VERSION__}</p>
</footer>

