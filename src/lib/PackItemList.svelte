<script>
	import { randomId } from '$lib';
	import PackItem from './PackItem.svelte';
	import { onMount } from 'svelte';
	import Sortable from 'sortablejs';
	import { loadPackingLists } from './backend';
	import {
		Button,
		ButtonGroup,
		Card,
		Input,
		Label,
		Listgroup,
		ListgroupItem,
		Modal
	} from 'flowbite-svelte';
	import { CirclePlusSolid, CloseOutline, FloppyDiskSolid, RectangleListSolid } from 'flowbite-svelte-icons';
	import MoveHandle from './MoveHandle.svelte';
	import EditModal from './EditModal.svelte';

	/** @type {ListItem[]} */
	export let list = [];

	/** @type {Map<string, ListItem>} */
	export let itemMap = new Map();

	/** @type {string} */
	export let group = 'group-' + randomId();

	let keys = list.map((item) => {
		const key = 'item-' + randomId();
		itemMap.set(key, item);
		return key;
	});
	console.log(itemMap);

	/** @type {Record<string, () => void>} */
	let destructors = {};

	const listId = 'list-' + randomId();

	/** @type {Record<string, boolean>} */
	let editModal = {};

	onMount(() => {
		let listElem = document.getElementById(listId);
		if (listElem !== null) {
			let sortable = Sortable.create(listElem, {
				group,
				draggable: '.pack-list-item',
				// handle: '.item-move-handle',
				animation: 150,
				invertSwap: true,
				emptyInsertThreshold: 20,
				onSort(event) {
					let rerender = false;
					if (event.from.id === listId && event.oldDraggableIndex !== undefined) {
						list.splice(event.oldDraggableIndex, 1);
						keys.splice(event.oldDraggableIndex, 1);
						rerender = true;
					}
					if (event.to.id === listId && event.newDraggableIndex !== undefined) {
						list.splice(event.newDraggableIndex, 0, itemMap.get(event.item.id));
						keys.splice(event.newDraggableIndex, 0, event.item.id);
						rerender = true;
					}
					if (rerender) {
						keys = keys;
					}
				}
			});
		}
	});

	function addItem() {
		/** @type {ListItem} */
		const newItem = {
			type: 'item',
			content: {
				name: 'Item',
				location: '',
				quantity: 1
			}
		};
		const key = 'item-' + randomId();
		list.push(newItem);
		keys.push(key);
		itemMap.set(key, newItem);
		editModal[key] = true;
		keys = keys;
	}

	function addCollection() {
		/** @type {ListItem} */
		const newCollection = {
			type: 'collection',
			content: {
				name: 'Collection',
				items: []
			}
		};
		const key = 'item-' + randomId();
		list.push(newCollection);
		keys.push(key);
		itemMap.set(key, newCollection);
		editModal[key] = true;
		keys = keys;
	}

	/** Recursively delete all child items of this list from itemMap. */
	export function destructor() {
		for (let key of keys) {
			const child = itemMap.get(key);
			if (child !== undefined) {
				if (child.type === 'collection' && key in destructors) {
					destructors[key]();
				}
				itemMap.delete(key);
			}
		}
	}

	/** @param {string} key */
	function deleteItem(key) {
		if (key in destructors) {
			destructors[key]();
			delete destructors[key];
			console.log(destructors);
		}
		delete editModal[key];
		const index = keys.findIndex((k) => k === key);
		list.splice(index, 1);
		keys.splice(index, 1);
		itemMap.delete(key);
		keys = keys;
	}
</script>

<div class="m-2 space-y-2">
<ul id={listId} class="space-y-2">
	{#each list.map((item, i) => ({ item, key: keys[i] })) as { item, key } (key)}
		{#if item.type === 'item'}
			<li class="rounded-s-none pack-list-item" id={key}>
				<PackItem
					handleClass="item-move-handle"
					bind:item={item.content}
					on:deleteItem={() => deleteItem(key)}
					bind:editModal={editModal[key]}
				/>
			</li>
		{:else}
			<li class="rounded-s-none pack-list-item">
				<Card padding="none" class="has-[>div>:focus]:outline">
					<div class="mt-2 mx-2 h-4 flex gap-2">
						<MoveHandle class="item-move-handle" />
						<button class="text-start w-full outline-none" on:click={() => (editModal[key] = true)}>
							<h5 class="my-0 text-sm font-semibold text-gray-900 dark:text-white leading-none">
								{item.content.name}
							</h5>
						</button>
					</div>
					<svelte:self bind:list={item.content.items} bind:itemMap bind:destructor={destructors[key]} {group} />
				</Card>

				<EditModal bind:open={editModal[key]} on:deleteItem={() => deleteItem(key)} focusId="name-input">
					<div class="space-y-2">
						<Label for="name-input">Name</Label>
						<Input
							id="name-input"
							type="text"
							placeholder="clothes"
							bind:value={item.content.name}
						/>
					</div>
				</EditModal>
			</li>
		{/if}
	{/each}
</ul>
<ButtonGroup>
	<Button color="primary" size="xs" on:click={addItem}>
		<CirclePlusSolid class="w-4 h-4 me-1" />
		Add item
	</Button>
	<Button color="primary" size="xs" on:click={addCollection}>
		<RectangleListSolid class="w-4 h-4 me-1" />
		Add collection
	</Button>
</ButtonGroup>
</div>