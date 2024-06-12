import { type Writable, writable } from "svelte/store";

type GlobalKeyStore = { event: KeyboardEvent; emitTime: Date };

export const globalKeyStore: Writable<null | GlobalKeyStore> = writable(null);
