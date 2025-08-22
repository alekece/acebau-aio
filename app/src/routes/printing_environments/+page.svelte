<script lang="ts">
	import type { PageProps } from './$types';
    import Badge from '$lib/components/ui/Badge.svelte';
    import Sun from '@lucide/svelte/icons/sun';
	import Status from '$lib/components/ui/Status.svelte';

	let { data }: PageProps = $props();

	console.log(data);
</script>

<div class="flex flex-col">
	<h1 class="preset-typo-title">Printing environment</h1>
	<p>
		Define the places where your printers run. Each environment has its own usage rhythm and
		electricity rate, so your cost estimates stay realistic—whether you print in France or anywhere
		else.
	</p>
	<div class="table-wrap">
		<table class="table">
			<thead>
				<tr>
					<th>Position</th>
					<th>Name</th>
					<th>Usage</th>
					<th>Electricity</th>
					<th class="!text-right">Status</th>
				</tr>
			</thead>
			<tbody class="[&>tr]:hover:preset-tonal-primary">
				{#each data.items as item, i}
					<tr>
						<td>{i}</td>
						<td>{item.name}</td>
						<td>{item.operating_factor}</td>
						<td>{item.electricity_cost_per_kwh}</td>
						<td class="text-right">
                            <Status value={item.status} />
                            <Status value="Drafted" />
                            <Status value="Archived" />
                        </td>
					</tr>
				{/each}
			</tbody>
			<tfoot>
				<tr>
					<td colspan="4">Total</td>
					<td class="text-right">{data.items.length} Elements</td>
				</tr>
			</tfoot>
		</table>
	</div>
</div>
