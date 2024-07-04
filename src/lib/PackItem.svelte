<script>
	import { Icon } from '@steeze-ui/svelte-icon';
	import { Label, Modal, ButtonGroup, Card, Input, Button, Popover } from 'flowbite-svelte';
	import { BellRingSolid, FloppyDiskSolid, TrashBinSolid } from 'flowbite-svelte-icons';
	import { createEventDispatcher } from 'svelte';
	import { TriangleExclamation } from '@steeze-ui/font-awesome';
	import { ExclamationTriangle } from '@steeze-ui/heroicons';
	import MoveHandle from './MoveHandle.svelte';
	import EditModal from './EditModal.svelte';

	/** @type {PackItem} */
	export let item;

	/** @type {string} */
	export let handleClass = '';

	export let editModal = false;
</script>

<Card horizontal padding="none" class="pl-2 has-[>:focus]:outline flex flex-row">
	<MoveHandle class={handleClass} />
	<button class="text-start outline-none px-2 py-1 grow" on:click={() => (editModal = true)}>
		<h5 class="my-0 text-sm font-semibold text-gray-900 dark:text-white leading-none">
			{item.name}
			<span class="font-normal">({item.quantity})</span>
		</h5>
		<span class="text-xs font-medium text-gray-500 dark:text-gray-400 leading-none">Location:</span>
		<span class="text-xs font-regular text-gray-900 dark:text-white leading-none"
			>{item.location}</span
		>
	</button>
</Card>

<EditModal bind:open={editModal} on:deleteItem focusId="name-input">
	<div class="space-y-2">
		<div class="flex">
			<Label for="name-input">Name</Label>
			<Label for="quantity-input" class="ml-auto w-20">Quantity</Label>
		</div>
		<ButtonGroup class="flex">
			<Input id="name-input" type="text" placeholder="shirt" required bind:value={item.name} />
			<Input id="quantity-input" type="number" required class="w-20" bind:value={item.quantity} />
		</ButtonGroup>
	</div>
	<div class="space-y-2">
		<Label for="location-input">Location</Label>
		<Input id="location-input" type="text" placeholder="packed" bind:value={item.location} />
	</div>
</EditModal>
