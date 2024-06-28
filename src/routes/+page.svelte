<script>
    import PackItem from '$lib/PackItem.svelte';
	import { ListGroup, ListGroupItem } from '@sveltestrap/sveltestrap';

    /** @type {import('./$types').PageData} */
    export let data;
    console.log(data);
</script>

<ListGroup class="tw-relative">
    {#each data.packingList as item}
        <ListGroupItem><PackItem {item} /></ListGroupItem>
    {/each}
</ListGroup>
