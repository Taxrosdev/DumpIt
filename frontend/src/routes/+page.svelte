<script lang="ts">
	import FileSelect from '../components/fileselect.svelte';
	import Card from '../components/card.svelte';
	import Button from '../components/button.svelte';
	import UploadCard from '../components/uploadcard.svelte';

	import type { Upload } from '../lib/upload.ts';

	let activeFile: File | null = $state(null);
	let uploadQueue: Upload[] = $state([]);
	$inspect(uploadQueue);

	// In KB
	let limit: number | null = $state(null);
	let isOverLimit = $derived(
		activeFile && limit ? (activeFile as File).size > limit * 1024 : false
	);

	$effect(() => {
		fetch('/api/meta/upload_limit').then(async (res) => {
			limit = parseInt(await res.text());
		});
	});

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
			filename: activeFile.name,
			err: null
		};

		uploadQueue.push(upload);
		upload = uploadQueue[uploadQueue.length - 1];

		fetch('/api/upload', {
			method: 'POST',
			body: formData
		})
			.then(async (res) => {
				let status = res.status;
				if (status == 200) {
					let url = await res.text();
					upload.uploaded = upload.size;
					upload.url = new URL(url, window.location.origin).href;
				} else {
					upload.err = await res.text();
				}
			})
			.catch((e) => {
				upload.err = 'Network Error: ' + e;
			});
	}
</script>

<div class="flex h-full w-screen grow items-center justify-center gap-10">
	<Card class="flex h-full gap-4 p-12">
		<div class="flex flex-col items-center justify-center gap-6 lg:w-1/2">
			<FileSelect
				{limit}
				{activeFile}
				{isOverLimit}
				class="fileselect flex h-40 w-full items-center justify-center overflow-clip rounded-md border-3 border-dashed transition"
				setActiveFile={(file) => {
					activeFile = file;
				}}
			/>
			<Button disabled={isOverLimit || !activeFile} onclick={upload} class="w-full">Upload</Button>
		</div>
		<div class="flex h-min max-h-full snap-y flex-col overflow-auto rounded-xl lg:max-w-1/2">
			{#each uploadQueue as upload (upload.id)}
				<div class="max-w-full snap-center odd:bg-uploadcard-odd even:bg-uploadcard-even">
					<UploadCard {upload} />
				</div>
			{/each}
		</div>
	</Card>
</div>
