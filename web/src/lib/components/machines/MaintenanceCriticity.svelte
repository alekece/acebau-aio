<script lang="ts">
	import Badge from '$lib/components/ui/Badge.svelte';
	import type { MachineMaintenanceCriticity } from '$lib/machine';

	let { criticity }: { criticity: MachineMaintenanceCriticity } = $props();

	const labels = { normal: 'À jour', due: 'À prévoir', critical: 'En retard' } as const;
	const tones = { normal: 'success', due: 'warning', critical: 'error' } as const;
</script>

<Badge small tonal tone={tones[criticity]}>{labels[criticity]}</Badge>
