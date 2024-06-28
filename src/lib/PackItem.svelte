<script>
	/** 
     * @typedef PackItem
     * @property {string} name
	 * @property {string} location
	 */

    /** @type {PackItem} */
	export let item;
    console.log(item);
    console.log($$props);
</script>

<div>
	<p>Name: {item?.name || 'N/A'}</p>
	<p>Location: {item?.location || 'N/A'}</p>
</div>
