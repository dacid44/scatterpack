<script>
	import {
		Button,
		Col,
		Container,
		Icon,
		Input,
		InputGroup,
		InputGroupText,
		Row
	} from '@sveltestrap/sveltestrap';
	import { createEventDispatcher } from 'svelte';

	/** @type {PackItem} */
	export let item;

	const dispatch = createEventDispatcher();

	function deleteItem() {
		dispatch('deleteItem');
	}
</script>

<Row>
	<Col xs="auto">
		<InputGroup size="sm">
			<InputGroupText>Name</InputGroupText>
			<Input type="text" bind:value={item.name} />
		</InputGroup>
	</Col>
	<Col xs="auto">
		<InputGroup size="sm">
			<InputGroupText>Location</InputGroupText>
			<Input type="text" bind:value={item.location} />
		</InputGroup>
	</Col>
	<Col xs="2">
		<InputGroup size="sm">
			<InputGroupText>Qty</InputGroupText>
			<Input type="number" bind:value={item.quantity} />
		</InputGroup>
	</Col>
	<Button class="tw-w-8" color="danger" size="sm" on:click={deleteItem}><Icon name="trash" /></Button>
</Row>
