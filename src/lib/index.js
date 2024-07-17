// place files you want to import through the `$lib` alias in this folder.

import { Observable } from 'rxjs';

/** @returns {string} */
export function randomId() {
	return Math.random().toString(16).slice(2);
}

/**
 * @template T
 * @param {import('svelte/store').Readable<T>} store
 * @returns {Observable<T>}
 */
export function observableFromStore(store) {
	return new Observable((observer) => {
		return store.subscribe((value) => observer.next(value));
	});
}
