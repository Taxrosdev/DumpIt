<script lang="ts">
	interface Props {
		class: string;
		activeFile: File | null;
		setActiveFile: (file: File) => void;
	}

	let { class: className, activeFile, setActiveFile }: Props = $props();

	let fileInput: HTMLInputElement;
	let dropzone: HTMLElement;

	let dragging = $state(false);

	$inspect(activeFile);
	$inspect(dragging);
</script>

<label
	class={'text-dropzone ' + className + (dragging ? '' : ' bg-transparent')}
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
		<p class="text-sm wrap-break-word text-dropzone-text">{activeFile.name}</p>
	{:else}
		<p class="text-sm text-dropzone-text">Drag a file here or click to upload a file</p>
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
