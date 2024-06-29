// place files you want to import through the `$lib` alias in this folder.

/** @returns {string} */
export function randomId() {
    return Math.random().toString(16).slice(2)
}