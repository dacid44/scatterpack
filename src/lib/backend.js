import { invoke } from '@tauri-apps/api/core';
import { readFile } from '@tauri-apps/plugin-fs';
import mime from 'mime';

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

/**
 * @returns {Promise<string>}
 */
export async function pickFile() {
	return await invoke('pick_file');
}

/**
 * @param {string} name
 * @returns {Promise<string>}
 */
export async function pickThumbnail(name) {
	return await invoke('pick_thumbnail', { name })
}

/**
 * Creates an object URL for a given thumbnail name and returns it, along with a destructor function
 * to be called once the image is loaded or otherwise no longer needed.
 *
 * @param {string} name
 * @returns {Promise<[string, () => void]>}
 */
export async function getThumbnailUrl(name) {
	/** @type {string} */
	let path = await invoke('get_thumbnail_path', { name })
	let type = mime.getType(path) || 'image';
	let blob = new Blob([await readFile(path)], { type });
	let url = window.URL.createObjectURL(blob);
	return [url, () => window.URL.revokeObjectURL(url)];
}

/** @returns {Promise<UniqueItem[]>} */
export async function loadUniqueItems() {
	return await invoke('load_unique_items');
}