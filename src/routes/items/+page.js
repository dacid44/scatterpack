import { loadUniqueItems } from '$lib/backend';

/** @type {import('./$types').PageLoad} */
export async function load() {
	return {
		uniqueItems: await loadUniqueItems()
	};
}
