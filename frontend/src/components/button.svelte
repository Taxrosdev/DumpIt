<script lang="ts">
	let { onclick, class: className = '', children, disabled = false } = $props();
</script>

<button
	class="rounded bg-button p-3 text-sm text-button-text {className} {disabled && 'bg-button/60'}"
	{disabled}
	{onclick}
>
	{@render children()}
</button>
