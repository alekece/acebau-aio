<script lang="ts">
	import Badge from '$lib/components/ui/Badge.svelte';
	import { Tone } from '$lib/types';

	type MachineState = 'available' | 'running' | 'maintenance' | 'broken';
	let { state }: { state: MachineState } = $props();

	const labels: Record<MachineState, string> = {
		available: 'Disponible',
		running: 'En production',
		maintenance: 'Maintenance',
		broken: 'En panne'
	};

	const tones: Record<MachineState, Tone> = {
		available: Tone.Success,
		running: Tone.Secondary,
		maintenance: Tone.Warning,
		broken: Tone.Error
	};

	let tone = $derived(tones[state]);
</script>

<Badge variant="tonal" {tone} class="[--badge-size:var(--text-sm)]">
	{#snippet icon()}
		<span class="size-2 rounded-full bg-current" aria-hidden="true"></span>
	{/snippet}
	{labels[state]}
</Badge>
