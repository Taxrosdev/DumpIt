<script lang="ts">
	import ProgressBar from './progressbar.svelte';
	import type { Upload } from '../lib/upload.ts';

	const { upload }: { upload: Upload } = $props();

	let copied = $state(false);
</script>

<button
	class="flex h-16 flex-col justify-around px-6 py-2 text-left"
	onclick={() => {
		if (upload.url) {
			navigator.clipboard.writeText(upload.url);
			copied = true;

			setTimeout(() => {
				copied = false;
			}, 1000);
		}
	}}
>
	<span class="truncate text-sm text-uploadcard-text">{upload.filename}</span>
	{#if upload.url}
		<span class="truncate text-xs text-uploadcard-text/75">
			{#if copied}
				Copied!
			{:else}
				Click to copy download link
			{/if}
		</span>
	{:else if upload.err}
		<span class="truncate text-xs text-uploadcard-text/75">
			Error! {upload.err}
		</span>
	{:else}
		<ProgressBar value={upload.uploaded} max={upload.size} />
	{/if}
</button>
