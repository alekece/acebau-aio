<script lang="ts">
	let date = new Date();
</script>

<footer>
	<p>© {date.getFullYear()} {__APP_NAME__} {__APP_VERSION__}</p>
</footer>

<style>
	footer {
		position: fixed;
		width: 100%;
		bottom: 0;
		background-color: #f8f9fa;
		padding: 1rem;
		text-align: center;
		font-size: 0.875rem;
		color: #6c757d;
	}
</style>
