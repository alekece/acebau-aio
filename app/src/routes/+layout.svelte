<script lang="ts">
	import '../app.css';
	import type { LayoutProps } from './$types';
    import { page } from '$app/state';
	import Menu from '$lib/components/Menu.svelte';
	import Footer from '$lib/components/Footer.svelte';

	let { data, children }: LayoutProps = $props();
    let activeItem = $derived(page.url.pathname);
</script>

<Menu items={data.links} />

{@render children()}

<Footer />
