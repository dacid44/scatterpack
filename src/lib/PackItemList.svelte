<script>
	import { randomId } from '$lib';
	import { Button, Icon, ListGroup, ListGroupItem } from '@sveltestrap/sveltestrap';
	import PackItem from './PackItem.svelte';
	import { onMount } from 'svelte';
	import Sortable from 'sortablejs';
	import { loadPackingLists } from './backend';

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

	const listId = 'list-' + randomId();

	onMount(() => {
		let listElem = document.getElementById(listId);
		if (listElem !== null) {
			let sortable = Sortable.create(listElem, {
				group,
				draggable: '.pack-list-item',
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
				name: '',
				location: '',
				quantity: 1
			}
		};
		const key = 'item-' + randomId();
		list.push(newItem);
		keys.push(key);
		itemMap.set(key, newItem);
		keys = keys;
	}

	/** @param {string} key */
	function deleteItem(key) {
		const index = keys.findIndex((k) => k === key);
		list.splice(index, 1);
		keys.splice(index, 1);
		itemMap.delete(key);
		keys = keys;
	}
</script>

<ListGroup id={listId}>
	{#each list.map((item, i) => ({ item, key: keys[i] })) as { item, key } (key)}
		{#if item.type === 'item'}
			<ListGroupItem class="tw-rounded-s-none pack-list-item" id={key}>
				<PackItem bind:item={item.content} on:deleteItem={() => deleteItem(key)} />
			</ListGroupItem>
		{:else}
			<ListGroupItem class="tw-rounded-s-none pack-list-item">
				<p>{item.content.name}</p>
				<div class="tw-border-solid tw-border-0 tw-border-l-8 tw-border-l-secondary">
					<svelte:self bind:list={item.content.items} bind:itemMap {group} />
				</div>
			</ListGroupItem>
		{/if}
	{/each}
	<ListGroupItem class="tw-border-0">
		<Button class="tw-w-8" color="primary" size="sm" on:click={addItem}>
			<Icon name="plus-lg" />
		</Button>
	</ListGroupItem>
</ListGroup>

<style>
	.pack-list-item :nth-last-child(2) {
		border-bottom-right-radius: inherit;
	}
</style>
