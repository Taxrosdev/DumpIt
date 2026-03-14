<script lang="ts">
	import FileSelect from '../components/fileselect.svelte';
	import Card from '../components/card.svelte';
	import Button from '../components/button.svelte';
	import UploadCard from '../components/uploadcard.svelte';

	import type { Upload } from '../lib/upload.ts';

	let activeFile: File | null = $state(null);
	let uploadQueue: Upload[] = $state([]);
	$inspect(uploadQueue);

	function upload() {
		if (!activeFile) {
			return;
		}

		const formData = new FormData();
		formData.append('file', activeFile);

		let upload: Upload = {
			id: Math.random().toString(),
			size: activeFile.size,
			uploaded: 0,
			url: null,
			filename: activeFile.name
		};

		uploadQueue.push(upload);
		upload = uploadQueue[uploadQueue.length - 1];

		fetch('/api/upload', {
			method: 'POST',
			body: formData
		}).then(async (res) => {
			let url = await res.text();
			upload.uploaded = upload.size;
			upload.url = new URL(url, window.location.origin).href;
		});
	}
</script>

<div class="flex h-full w-screen grow items-center justify-center gap-10 transition">
	<Card class="items-center gap-4">
		<FileSelect
			{activeFile}
			class="fileselect flex h-40 w-70 items-center justify-center overflow-clip rounded-md border-3 border-dashed transition"
			setActiveFile={(file) => {
				activeFile = file;
			}}
		/>
		<Button class="" onclick={upload}>Upload</Button>
	</Card>
	<Card class="">
		{#each uploadQueue as upload (upload.id)}
			<div>
				<UploadCard {upload} />
			</div>
		{/each}
	</Card>
</div>
