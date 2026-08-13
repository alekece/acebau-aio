<script lang="ts">
	import { Switch } from '@skeletonlabs/skeleton-svelte';
	import MoonStar from '@lucide/svelte/icons/moon-star';
	import Sun from '@lucide/svelte/icons/sun';

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
		document.documentElement.setAttribute('data-mode', localStorage.getItem('mode') || 'light');
	</script>
</svelte:head>

<Switch {checked} {onCheckedChange}>
	<Switch.Control>
		<Switch.Thumb>
			{#if checked}
				<MoonStar size={12} />
			{:else}
				<Sun size={12} />
			{/if}
		</Switch.Thumb>
	</Switch.Control>
	<Switch.HiddenInput />
</Switch>
