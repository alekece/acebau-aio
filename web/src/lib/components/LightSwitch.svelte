<script lang="ts">
	import { onMount } from 'svelte';
	import { Switch } from '@skeletonlabs/skeleton-svelte';
	import MoonStar from '@lucide/svelte/icons/moon-star';
	import Sun from '@lucide/svelte/icons/sun';

	let checked = $state(false);

	onMount(() => {
		const mode = localStorage.getItem('mode') || 'light';
		checked = mode === 'dark';
		document.documentElement.setAttribute('data-mode', mode);
	});

	const onCheckedChange = (event: { checked: boolean }) => {
		const mode = event.checked ? 'dark' : 'light';
		document.documentElement.setAttribute('data-mode', mode);
		localStorage.setItem('mode', mode);
		checked = event.checked;
	};
</script>

<Switch name="mode" {checked} {onCheckedChange} class="inline-flex items-center">
	<Switch.HiddenInput />
	<Switch.Control
		class="relative inline-flex h-7 w-12 items-center rounded-full bg-surface-300-700 p-1 transition data-[state=checked]:bg-primary-500"
	>
		<Switch.Thumb
			class="grid size-5 place-items-center rounded-full bg-surface-50-950 shadow-sm transition-transform data-[state=checked]:translate-x-5"
		>
			{#if checked}<MoonStar size="13" />{:else}<Sun size="13" />{/if}
		</Switch.Thumb>
	</Switch.Control>
	<Switch.Label><span class="sr-only">Changer de thème</span></Switch.Label>
</Switch>
