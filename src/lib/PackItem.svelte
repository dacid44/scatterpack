<script>
	import {
		Label,
		ButtonGroup,
		Card,
		Input,
		Checkbox,
		Progressbar,
		Button,
		Gallery
	} from 'flowbite-svelte';
	import EditModal from './EditModal.svelte';

	/** @type {PackItem} */
	export let item;

	export let editModal = false;

	let completeRatio = 0;
	$: completeRatio = Math.min((item.packed.length / item.quantity) * 100, 100);

	let complete = false;
	$: complete = item.packed.length >= item.quantity;

	let partially_complete = false;
	$: partially_complete = item.packed.length > 0 && item.packed.length < item.quantity;

	function addItem() {
		item.packed.push({ type: 'generic' });
		item = item;
	}

	function removeItem() {
		const lastGenericItem = item.packed.findLastIndex((e) => e.type === 'generic');
		if (lastGenericItem > -1) {
			item.packed.splice(lastGenericItem, 1);
			item = item;
		}
	}

	/** @param {MouseEvent} event */
	function handleCheckbox(event) {
		if (complete) {
			item.packed = item.packed.filter((e) => e.type !== 'generic');
		} else {
			while (item.packed.length < item.quantity) {
				item.packed.push({ type: 'generic' });
			}
			item = item;
		}
	}
</script>

<Card horizontal padding="none" class="pl-2 has-[>:focus]:outline flex flex-row ">
	<Checkbox checked={complete} indeterminate={partially_complete} on:click={handleCheckbox} />
	<button class="text-start outline-none px-2 py-1 grow" on:click={() => (editModal = true)}>
		<h5 class="my-0 text-sm font-semibold text-gray-900 dark:text-white leading-none">
			{item.name}
			<span class="font-normal">({item.quantity})</span>
		</h5>
		<span class="text-xs font-medium text-gray-500 dark:text-gray-400 leading-none">
			Location:
		</span>
		<span class="text-xs font-regular text-gray-900 dark:text-white leading-none">
			{item.location}
		</span>
	</button>
	{#if item.quantity > 1}
		<div class="w-24 m-1 gap-1 flex flex-col">
			<div class="flex flex-row items-center">
				<span class="text-sm">{item.packed.length}/{item.quantity}</span>
				<ButtonGroup class="ml-auto">
					<Button
						pill
						size="xs"
						class="py-0 px-2 text-xs leading-none"
						disabled={item.packed.every((e) => e.type !== 'generic')}
						on:click={removeItem}
					>
						-
					</Button>
					<Button
						pill
						size="xs"
						class="py-1 px-2 text-xs leading-none"
						on:click={addItem}
					>
						+
					</Button>
				</ButtonGroup>
			</div>
			<Progressbar progress={completeRatio} />
		</div>
	{/if}
</Card>

<EditModal bind:open={editModal} on:deleteItem focusId="name-input">
	<div class="space-y-2">
		<div class="flex">
			<Label for="name-input">Name</Label>
			<Label for="quantity-input" class="ml-auto w-20">Quantity</Label>
		</div>
		<ButtonGroup class="flex">
			<Input
				id="name-input"
				type="text"
				placeholder="shirt"
				required
				bind:value={item.name}
			/>
			<Input
				id="quantity-input"
				type="number"
				required
				class="w-20"
				bind:value={item.quantity}
			/>
		</ButtonGroup>
	</div>
	<div class="space-y-2">
		<Label for="location-input">Location</Label>
		<Input id="location-input" type="text" placeholder="packed" bind:value={item.location} />
	</div>
	<div class="space-y-2">
		<Label>Unique items</Label>
		<Gallery class="grid-cols-2 sm:grid-cols-3">
			<Card>Lorem ipsum dolor sit amet.</Card>
			<Card>Cum, laudantium! Non, quae velit.</Card>
			<Card>Consequatur fugit culpa praesentium laudantium?</Card>
			<Card>Incidunt error alias eius delectus.</Card>
		</Gallery>
	</div>
</EditModal>
