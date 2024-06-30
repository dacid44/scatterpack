<script>
	import { savePackingList } from '$lib/backend';
	import PackItemList from '$lib/PackItemList.svelte';
	import { Button } from 'flowbite-svelte';

	/** @type {import('./$types').PageData} */
	export let data;

	async function saveList() {
        console.log(data.packingList);
		await savePackingList({
			name: 'example',
			items: data.packingList
		});
	}
</script>

<PackItemList bind:list={data.packingList}></PackItemList>
<Button color="primary" on:click={saveList}>Save</Button>
<a href="/items">items</a>