import { type Writable, writable } from "svelte/store";

export const selectedVault: Writable<null | string> = writable(null);
