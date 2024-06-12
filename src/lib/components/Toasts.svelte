<script lang="ts">
  import escape from "escape-html";
  import sanitizeHtml from "sanitize-html";
  import { flip } from "svelte/animate";
  import { fly } from "svelte/transition";

  import FilledIcon from "./icons/FilledIcon.svelte";

  import { type ToastType, toasts } from "$lib/stores/toasts";

  const duration = 200;

  const getToastColor = (type: ToastType) => {
    switch (type) {
      case "error": {
        return "bg-red-600/25 dark:bg-red-400/25 text-red-800 dark:text-red-200";
      }

      case "info": {
        return "bg-cyan-600/25 dark:bg-cyan-400/25 text-cyan-800 dark:text-cyan-200";
      }

      case "success": {
        return "bg-green-600/25 dark:bg-green-400/25 text-green-800 dark:text-green-200";
      }

      case "warning": {
        return "bg-orange-600/25 dark:bg-orange-400/25 text-orange-800 dark:text-orange-200";
      }
    }
  };

  const convertSemiMarkdownToHtml = (markdown: string) => {
    return sanitizeHtml(
      escape(markdown)
        .replaceAll(/(\*\*\*(.*?)\*\*\*)/gm, "<i><b>$2</b></i>")
        .replaceAll(/(\*\*(.*?)\*\*)/gm, "<b>$2</b>")
        .replaceAll(/(\*(.*?)\*)/gm, "<i>$2</i>"),
    );
  };
</script>

<div class="fixed right-0 top-4 z-10 flex flex-col gap-2">
  {#each $toasts as toast, index (toast.id)}
    <div
      class={`
        flex w-64 justify-between rounded-l p-3 backdrop-blur

        ${getToastColor(toast.type)}
      `}
      in:fly={{ x: 200, duration }}
      out:fly={{ x: 200, duration }}
      animate:flip={{ duration }}
    >
      <p class="overflow-hidden break-words">
        {@html convertSemiMarkdownToHtml(toast.message)}
      </p>
      <button on:click={() => toasts.removeAtIndex(index)}>
        <FilledIcon class="pointer-events-none" icon="Close" />
      </button>
    </div>
  {/each}
</div>
