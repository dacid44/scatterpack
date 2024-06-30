import { loadPackingLists } from '$lib/backend';
import { invoke } from '@tauri-apps/api/core';

/** @type {import('./$types').PageLoad} */
export async function load() {
	return {
		packingList: (await loadPackingLists()).find(list => list.name === 'example')?.items || []
		// /** @type {ListItem[]} */
		// packingList: [
		// 	{
		// 		type: 'item',
		// 		content: {
		// 			name: 'asdf',
		// 			location: 'asdf',
		// 			quantity: 3
		// 		}
		// 	}
		// ]
	};
}
