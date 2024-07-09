<script>
	import { CirclePlusSolid, EditSolid } from 'flowbite-svelte-icons';
	import { getThumbnailUrl, pickThumbnail } from './backend';
	import { randomId } from '$lib';
	import { Button } from 'flowbite-svelte';

	/** @type {string | null} */
	export let name = null;

	/** @type {string | null} */
	let imageUrl = null;
	let imageDestructor = () => {};

	$: {
		if (name === null) {
			imageUrl = null;
			imageDestructor = () => {};
		} else {
			getThumbnailUrl(name).then((value) => ([imageUrl, imageDestructor] = value));
		}
	}

	async function selectImage() {
		name = await pickThumbnail('thumbnail-' + randomId());
	}
</script>

{#if imageUrl === null}
	<button on:click={selectImage} class="thumbnail-upload no-upload">
		<div class="thumbnail-upload-inner">
			<CirclePlusSolid />
			<span class="text-xs text-center">Choose an image</span>
		</div>
	</button>
{:else}
	<button on:click={selectImage} class="thumbnail-upload uploaded border-solid">
		<img
			src={imageUrl}
			on:load={imageDestructor}
			class="absolute inset-0 size-full object-cover"
			alt="uploaded thumbnail"
		/>
		<div class="thumbnail-upload-inner hidden">
			<EditSolid />
			<span class="text-xs text-center font-semibold z-10">Change image</span>
			<Button on:click={() => (name = null)} size="xs" color="red" pill class="h-6 mt-auto">
				Remove
			</Button>
		</div>
	</button>
{/if}

<style>
	.thumbnail-upload {
		@apply size-24 relative;
		@apply cursor-pointer overflow-hidden;
		@apply rounded-lg border-2 border-gray-300 dark:border-gray-600 dark:hover:border-gray-500;
	}

	.thumbnail-upload.no-upload {
		@apply text-gray-500 dark:text-gray-400;
		@apply border-dashed;
		@apply bg-gray-50 dark:bg-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600;
	}

	.thumbnail-upload-inner {
		@apply relative w-full h-full pt-2 pb-1;
		@apply flex flex-col justify-center items-center;
	}

	.uploaded:is(:hover, :focus-within) > .thumbnail-upload-inner {
		@apply !flex;
		/* TODO: decide if the colors should just stay the same in dark mode */
		@apply text-white dark:text-gray-700;
		@apply bg-[var(--bg-mask-lightmode)] dark:bg-[var(--bg-mask-darkmode)];
		--bg-mask-lightmode: hsla(180, 0%, 25%, 0.75);
		--bg-mask-darkmode: hsla(180, 0%, 75%, 0.75);
	}
</style>
