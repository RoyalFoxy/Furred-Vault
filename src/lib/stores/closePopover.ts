import { derived } from "svelte/store";

import { globalKeyStore } from "./globalKeyStore";
import { globalMouseStore } from "./globalMouseStore";

import { LAYOUT_CONTAINER } from "$lib/constants";

export const closePopover = derived(
  [globalKeyStore, globalMouseStore],
  ([$globalKeyStore, $globalMouseStore]) => {
    const keyTime = $globalKeyStore?.emitTime.getTime() || 0;
    const mouseTime = $globalMouseStore?.emitTime.getTime() || 0;

    const target = $globalMouseStore?.event.target as Element | null | undefined;

    const shouldClose =
      keyTime > mouseTime
        ? $globalKeyStore?.event.code === "Escape"
        : target && isOrIsUnderLayoutContainer(target);

    return shouldClose;
  },
  false,
);

function isOrIsUnderLayoutContainer(element: Element) {
  return element.id === LAYOUT_CONTAINER || element.parentElement?.id === LAYOUT_CONTAINER;
}

export function resetClosePopover() {
  globalKeyStore.set(null);
  globalMouseStore.set(null);
}
