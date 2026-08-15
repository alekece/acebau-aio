<script lang="ts" generics="T extends string">
	import ChevronDown from '@lucide/svelte/icons/chevron-down';
	import { Menu, Portal } from '@skeletonlabs/skeleton-svelte';
	import Badge from './Badge.svelte';

	type SelectableBadgeOption = { value: T; label: string; tone?: string | null };
	type SelectableBadgeSize = 'sm' | 'base';

	let {
		value,
		options,
		onChange,
		ariaLabel,
		label,
		size = 'base',
		disabled = false
	}: {
		value: T;
		options: ReadonlyArray<SelectableBadgeOption>;
		onChange: (value: T) => void | Promise<void>;
		ariaLabel: string;
		label?: string;
		size?: SelectableBadgeSize;
		disabled?: boolean;
	} = $props();

	let pending = $state(false);
	let selected = $derived(options.find((option) => option.value === value) ?? options[0]);
	const badgeClass = 'pointer-events-none gap-1 [--badge-size:var(--text-sm)]';
	let contentClass = $derived(
		size === 'sm'
			? 'w-max min-w-36 rounded-container !border-0 bg-surface-50-950 p-1 shadow-xl outline-none'
			: 'w-max min-w-40 rounded-container !border-0 bg-surface-50-950 p-1.5 shadow-xl outline-none'
	);
	let labelClass = $derived(
		size === 'sm'
			? 'px-2 py-1 text-[11px] font-semibold text-surface-500-400'
			: 'px-2.5 py-1.5 text-xs font-semibold text-surface-500-400'
	);
	let itemClass = $derived(
		size === 'sm'
			? 'flex cursor-pointer items-center whitespace-nowrap rounded-base py-1 pr-0 pl-2 transition-colors outline-none data-[highlighted]:bg-surface-100-900'
			: 'flex cursor-pointer items-center whitespace-nowrap rounded-base py-2 pr-0 pl-2.5 transition-colors outline-none data-[highlighted]:bg-surface-100-900'
	);

	async function selectValue(nextValue: T) {
		if (nextValue === value || pending) return;

		pending = true;
		try {
			await onChange(nextValue);
		} finally {
			pending = false;
		}
	}
</script>

{#if selected}
	<Menu positioning={{ placement: 'bottom-start', gutter: 6 }}>
		<Menu.Trigger
			class="rounded-full ring-0 transition outline-none focus:ring-0 focus:outline-none focus-visible:ring-0 focus-visible:brightness-90 focus-visible:outline-none enabled:hover:brightness-95 disabled:cursor-wait disabled:opacity-60"
			disabled={disabled || pending}
			aria-label={ariaLabel}
		>
			<Badge variant="tonal" tone={selected.tone} class={badgeClass}>
				<span class="inline-flex items-center gap-1">
					{selected.label}
					<ChevronDown size={size === 'sm' ? 12 : 13} strokeWidth={2.25} aria-hidden="true" />
				</span>
			</Badge>
		</Menu.Trigger>
		<Portal>
			<Menu.Positioner class="z-[100]">
				<Menu.Content class={contentClass}>
					{#if label}<p class={labelClass}>{label}</p>{/if}
					{#each options as option (option.value)}
						<Menu.OptionItem
							type="radio"
							value={option.value}
							checked={value === option.value}
							onCheckedChange={(checked) => {
								if (checked) void selectValue(option.value);
							}}
							class={itemClass}
						>
							<Menu.ItemText>
								<Badge
									variant="tonal"
									tone={option.tone}
									class="{badgeClass} {value === option.value ? 'font-bold' : ''}"
								>
									{option.label}
								</Badge>
							</Menu.ItemText>
						</Menu.OptionItem>
					{/each}
				</Menu.Content>
			</Menu.Positioner>
		</Portal>
	</Menu>
{/if}
