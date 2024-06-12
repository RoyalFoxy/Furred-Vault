import { type Writable, writable } from "svelte/store";

type GlobalMouseStore = { event: MouseEvent; emitTime: Date };

export const globalMouseStore: Writable<null | GlobalMouseStore> = writable(null);
