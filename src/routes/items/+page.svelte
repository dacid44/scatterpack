<script>
	import { pickFile } from '$lib/backend';
	import { Button } from 'flowbite-svelte';
	import { open } from '@tauri-apps/plugin-dialog';
	import { readFile } from '@tauri-apps/plugin-fs';
	import UniqueItem from '$lib/UniqueItem.svelte';

	let url = '';

	async function openFileDialog() {
		let path = await pickFile();
		let imageBlob = new Blob([await readFile(path)], { type: 'image/jpg' });
		url = window.URL.createObjectURL(imageBlob);
	}

	let jsPath = '';
	async function openFileDialogJs() {
		jsPath = (
			await open({
				multiple: false,
				directory: false
			})
		)?.path;
	}

	let item = {
		name: 'choir tour shirt',
		description: 'Gray t-shirt with pink cherry blossom design',
		location: 'packed',
		thumbnail: null
	};
</script>

<Button color="primary" on:click={openFileDialog}>open file picker</Button>
<img src={url} on:load={() => window.URL.revokeObjectURL(url)} alt="user selected" />
<Button color="primary" on:click={openFileDialogJs}>open file picker (JS)</Button>
<div>{jsPath}</div>

<UniqueItem {item} />
