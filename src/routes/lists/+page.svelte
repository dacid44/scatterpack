<script>
	import { beforeNavigate } from '$app/navigation';
	import { observableFromStore } from '$lib';
	import { savePackingList, updatePackingList } from '$lib/backend';
	import PackItemList from '$lib/PackItemList.svelte';
	import { Button } from 'flowbite-svelte';
	import { debounceTime } from 'rxjs';
	import { writable } from 'svelte/store';

	/** @type {import('./$types').PageData} */
	export let data;

	let packingList = writable(data.packingList);
	let packingListObservable = observableFromStore(packingList);

	// If no inputs for a bit of time, update the data held in the backend
	packingListObservable.pipe(debounceTime(500)).subscribe(async (items) => {
		await updatePackingList({ name: 'example', items });
	});

	// If no inputs for longer, or on navigate away, update the data on disk
	packingListObservable.pipe(debounceTime(5000)).subscribe(async (items) => {
		await savePackingList({ name: 'example', items });
	});
	beforeNavigate(async () => {
		await savePackingList({ name: 'example', items: $packingList });
	});
</script>

<PackItemList bind:list={$packingList}></PackItemList>
