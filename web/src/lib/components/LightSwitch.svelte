<script lang="ts">
	import { Switch } from '@skeletonlabs/skeleton-svelte';
	import MoonStar from '@lucide/svelte/icons/moon-star';
	import Sun from '@lucide/svelte/icons/sun';

	let props = $props();

	let checked = $state(false);

	$effect(() => {
		const mode = localStorage.getItem('mode') || 'light';
		checked = mode === 'dark';
	});

	const onCheckedChange = (event: { checked: boolean }) => {
		const mode = event.checked ? 'dark' : 'light';
		document.documentElement.setAttribute('data-mode', mode);
		localStorage.setItem('mode', mode);
		checked = event.checked;
	};
</script>

<svelte:head>
	<script>
		const mode = localStorage.getItem('mode') || 'light';
		document.documentElement.setAttribute('data-mode', mode);
	</script>
</svelte:head>

<Switch name="mode" controlActive="bg-surface-200" {checked} {onCheckedChange}>
	{#snippet activeChild()}<MoonStar size="14" />{/snippet}
	{#snippet inactiveChild()}<Sun size="14" />{/snippet}
</Switch>
