<script>
	import { getThumbnailUrl, pickFile, saveThumbnailFile } from '$lib/backend';
	import { Button } from 'flowbite-svelte';
	import { open } from '@tauri-apps/plugin-dialog';
	import { readFile } from '@tauri-apps/plugin-fs';
	import UniqueItem from '$lib/UniqueItem.svelte';
	import { randomId } from '$lib';

	/** @type {import('./$types').PageData} */
	export let data;

	let length = 50;
</script>

<ul>
	{#each data.uniqueItems as item}
		<UniqueItem {item} />
	{/each}
</ul>
