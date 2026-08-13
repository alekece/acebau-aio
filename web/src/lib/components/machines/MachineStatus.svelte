<script lang="ts">
	import Badge from '$lib/components/ui/Badge.svelte';
	import { Tone } from '$lib/components/ui/presets';

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
</script>

<Badge variant="tonal" tone={tones[state]} class="[--badge-size:var(--text-sm)]">
	{labels[state]}
</Badge>
