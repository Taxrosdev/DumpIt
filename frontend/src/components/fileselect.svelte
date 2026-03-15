<script lang="ts">
	interface Props {
		class: string;
		activeFile: File | null;
		setActiveFile: (file: File) => void;
		limit: number | null;
		isOverLimit: boolean;
	}

	let { class: className, activeFile, setActiveFile, limit, isOverLimit }: Props = $props();

	let fileInput: HTMLInputElement;
	let dropzone: HTMLElement;

	let dragging = $state(false);
</script>

<label
	class="flex flex-col gap-1 text-dropzone {isOverLimit &&
		' bg-overlimit/25 text-overlimit'} {className} {dragging ? '' : ' bg-transparent'}"
	bind:this={dropzone}
	ondragenter={(_e) => {
		dragging = true;
	}}
	ondragover={(e) => {
		e.preventDefault();
		dragging = true;
	}}
	ondragleave={() => {
		dragging = false;
	}}
	ondrop={(e) => {
		e.preventDefault();
		dragging = false;
		let file = e.dataTransfer?.files[0];
		if (file) {
			setActiveFile(file);
		}
	}}
	role="region"
	aria-label="File upload Dropzone"
>
	<!-- For opening the File Selector -->
	<input
		type="file"
		bind:this={fileInput}
		onchange={(e) => {
			let file = e.currentTarget.files?.[0];
			if (file != null) {
				setActiveFile(file);
			}
		}}
	/>

	{#if activeFile}
		<p
			class="w-5/6 truncate text-center text-sm text-dropzone-text {isOverLimit &&
				'text-overlimit/80'}"
		>
			{activeFile.name}
		</p>
	{:else}
		<p class="text-sm text-dropzone-text">Drag a file here or click to upload a file</p>
	{/if}

	{#if limit}
		<p class="text-sm text-dropzone-text {isOverLimit && 'text-overlimit/80'}">
			Upload Limit: {limit / 1024}MB
		</p>
	{:else}
		<p class="text-sm"></p>
	{/if}
</label>

<style>
	input {
		/* Apparently needed for accessibility? */
		position: absolute;
		opacity: 0;

		/* dropzone annoyance. */
		pointer-events: none;
	}
</style>
