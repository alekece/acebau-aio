<script lang="ts">
	import TriangleAlert from '@lucide/svelte/icons/triangle-alert';

	let { visible = false, message = '' }: { visible?: boolean; message?: string | null } = $props();
</script>

{#if visible}
	<div
		class="mb-5 flex items-start gap-3 rounded-base border border-warning-300-700 preset-tonal-warning p-3"
		role="status"
	>
		<TriangleAlert class="mt-0.5 shrink-0" size={18} />
		<div>
			<strong class="block text-sm">Données de démonstration</strong>
			<span class="text-xs"
				>{message ||
					'L’API est indisponible ; les données temporaires proviennent du loader de la page.'}</span
			>
		</div>
	</div>
{/if}
