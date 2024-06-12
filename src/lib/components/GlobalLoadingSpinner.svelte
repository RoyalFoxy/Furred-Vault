<script lang="ts">
  import { fade } from "svelte/transition";

  import Throbber from "$lib/components/Throbber.svelte";
  import { globalLoadingSpinner } from "$lib/stores/globalLoadingSpinner";

  const delay = 300;

  let timeout: number | null = null;

  let show = false;

  $: if ($globalLoadingSpinner) {
    timeout = setTimeout(() => (show = true), delay);
  } else {
    if (timeout) {
      clearTimeout(timeout);
    }
    show = false;
  }

  timeout;
</script>

{#if show}
  <div
    class="
      fixed left-[50%] top-[50%] z-30 size-24 -translate-x-[50%]
      -translate-y-[50%] rounded-full bg-zinc-800/25 p-4 backdrop-blur
    "
    in:fade
    out:fade
  >
    <Throbber class="size-full" />
  </div>
{/if}
