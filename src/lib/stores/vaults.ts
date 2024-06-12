import { type Writable, writable } from "svelte/store";

export const vaults: Writable<null | string[]> = writable(null);
