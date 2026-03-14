<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import '@fontsource-variable/inter/wght.css';
	import Footer from '../components/footer.svelte';

	let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div class="flex h-screen flex-col bg-background-main">
	<div class="flex-1">
		{@render children()}
	</div>
	<Footer />
</div>
