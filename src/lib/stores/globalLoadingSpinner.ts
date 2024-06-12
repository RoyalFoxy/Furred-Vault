import { writable } from "svelte/store";

function loadingStore() {
  const { subscribe, update, set } = writable(false);

  return {
    subscribe,
    toggle: () => update((current) => !current),
    set,
    off: () => set(false),
    on: () => set(true),
  };
}

export const globalLoadingSpinner = loadingStore();
