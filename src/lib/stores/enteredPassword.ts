import { type Writable, writable } from "svelte/store";

export const enteredPassword: Writable<null | string> = writable(null);
