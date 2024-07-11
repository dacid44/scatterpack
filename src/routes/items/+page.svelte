<script>
	import { pickFile } from '$lib/backend';
	import { Button } from 'flowbite-svelte';
	import { open } from '@tauri-apps/plugin-dialog';
	import { readFile } from '@tauri-apps/plugin-fs';
	import UniqueItem from '$lib/UniqueItem.svelte';

	/** @type {import('./$types').PageData} */
	export let data;
</script>

<ul>
	{#each data.uniqueItems as item}
	    <UniqueItem {item} />
	{/each}
</ul>
