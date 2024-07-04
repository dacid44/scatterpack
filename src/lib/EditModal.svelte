<script>
	import { Button, ButtonGroup, Modal } from 'flowbite-svelte';
	import { BellRingSolid, FloppyDiskSolid, TrashBinSolid } from 'flowbite-svelte-icons';
	import { afterUpdate, createEventDispatcher, onMount } from 'svelte';

	/** @type {boolean} */
	export let open = false;
	/** @type {string} */
	export let title = 'Edit item';

	/** @type {string | null} */
	export let focusId = null;

	const dispatch = createEventDispatcher();

	let attemptDelete = false;
    let ranAutofocus = false;

	function deleteItem() {
		if (attemptDelete) {
			dispatch('deleteItem');
		} else {
			attemptDelete = true;
		}
	}

	$: {
		if (!open) {
			attemptDelete = false;
            ranAutofocus = false;
		}
	}

	afterUpdate(() => {
		if (open && !ranAutofocus && focusId !== null) {
			const focusElement = document.getElementById(focusId);
			if (focusElement) {
				focusElement.focus();
			}
            ranAutofocus = true;
		}
	});

    /** @param {MouseEvent} event*/
    function closeButton(event) {
        event.preventDefault();
        open = false;
    }
</script>

<Modal bind:open>
	<form class="flex flex-col space-y-4" action="#">
		<h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">{title}</h3>
		<slot />
		<ButtonGroup>
			<Button type="submit" color="primary" on:click={closeButton}>
				<FloppyDiskSolid class="mr-1" />
				Save
			</Button>
			<!-- TODO: make this work with keyboard navigation, maybe using on:blur? -->
			<Button
				type="button"
				color="red"
				on:click={deleteItem}
				on:mouseleave={() => (attemptDelete = false)}
			>
				{#if attemptDelete}
					<BellRingSolid class="mr-1" />
					Are you sure?
				{:else}
					<TrashBinSolid class="mr-1" />
					Delete
				{/if}
			</Button>
		</ButtonGroup>
	</form>
</Modal>
