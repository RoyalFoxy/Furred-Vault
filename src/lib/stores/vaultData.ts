import { type Writable, writable } from "svelte/store";

import type { VaultData } from "$lib/types/generated/VaultData";

export const vaultData: Writable<null | VaultData> = writable(null);
