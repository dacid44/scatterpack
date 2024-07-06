<script>
	import { pickFile } from '$lib/backend';
	import { Button } from 'flowbite-svelte';
	import { open } from '@tauri-apps/plugin-dialog';

	let path = '';

	async function openFileDialog() {
		path = await pickFile();
	}

	let jsPath = '';
	async function openFileDialogJs() {
		jsPath = (await open({
			multiple: false,
			directory: false,
		}))?.path;
	}
</script>

<Button color="primary" on:click={openFileDialog}>open file picker</Button>
<div>{path}</div>
<Button color="primary" on:click={openFileDialogJs}>open file picker (JS)</Button>
<div>{jsPath}</div>
