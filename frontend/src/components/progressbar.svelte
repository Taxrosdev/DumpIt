<script lang="ts">
	const { value, max } = $props();

	const percentage = $derived((value / max) * 100);
</script>

<div class="h-2 w-full rounded-full bg-uploadcard-progressbar-background">
	<div class="h-2 rounded-full bg-uploadcard-progressbar-main" style="width: {percentage}%;"></div>
</div>
