<script>
	import { Label, Card, Input, Textarea } from 'flowbite-svelte';
	import MoveHandle from './MoveHandle.svelte';
	import EditModal from './EditModal.svelte';
	import ThumbnailUpload from './ThumbnailUpload.svelte';

	/** @type {UniqueItem} */
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
		<span class="text-xs font-medium text-gray-500 dark:text-gray-400 leading-none"
			>Location:</span
		>
		<span class="text-xs font-regular text-gray-900 dark:text-white leading-none"
			>{item.location}</span
		>
	</button>
</Card>

<EditModal bind:open={editModal} on:deleteItem focusId="name-input">
	<div class="flex flex-col sm:flex-row-reverse items-center gap-4 space-y-2">
		<ThumbnailUpload bind:name={item.thumbnail} />
		<div class="relative grow w-full sm:w-auto space-y-2">
			<Label for="name-input" class="absolute top-[-1.25rem]">Name</Label>
			<Input
				id="name-input"
				type="text"
				placeholder="shirt"
				required
				bind:value={item.name}
			/>
		</div>
	</div>
	<div class="space-y-2">
		<Label for="description-input">Description</Label>
		<Textarea id="description-input" rows="3" bind:value={item.description} />
	</div>
	<div class="space-y-2">
		<Label for="location-input">Location</Label>
		<Input id="location-input" type="text" placeholder="packed" bind:value={item.location} />
	</div>
</EditModal>
