<script lang="ts">
	let { onclick, class: className, children } = $props();
</script>

<button class={'rounded bg-button p-3 text-sm text-button-text ' + className} {onclick}>
	{@render children()}
</button>
