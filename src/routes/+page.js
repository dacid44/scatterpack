import { invoke } from '@tauri-apps/api/core';

/** @type {import('./$types').PageLoad} */
export async function load() {
	return {
		packingList: await invoke('packing_list')
	};
}
