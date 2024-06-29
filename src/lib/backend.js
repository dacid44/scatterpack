import { invoke } from '@tauri-apps/api/core';

/**
 * @returns {Promise<PackingList>}
 */
export async function examplePackingList() {
	return await invoke('example_packing_list');
}

/**
 * @param {PackingList} packingList
 */
export async function savePackingList(packingList) {
	await invoke('save_packing_list', { packingList });
}

/**
 * @returns {Promise<PackingList[]>}
 */
export async function loadPackingLists() {
	return await invoke('load_packing_lists');
}
