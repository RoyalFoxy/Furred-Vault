<script lang="ts">
  export let style: "base" | "success" | "destructive" = "base";

  export let fullyRounded: boolean = false;

  const getColorClasses = () => {
    switch (style) {
      case "base": {
        return "bg-zinc-900 text-zinc-100 active:bg-zinc-700 active:text-zinc-300 dark:bg-zinc-100 dark:text-zinc-900 dark:hover:bg-zinc-200 dark:hover:text-zinc-800 dark:active:bg-zinc-300 dark:active:text-zinc-700 hover:bg-zinc-800 hover:text-zinc-200";
      }

      case "success": {
        return "bg-green-900 text-green-100 active:bg-green-700 active:text-green-300 dark:bg-green-100 dark:text-green-900 dark:hover:bg-green-200 dark:hover:text-green-800 dark:active:bg-green-300 dark:active:text-green-700 hover:bg-green-800 hover:text-green-200";
      }

      case "destructive": {
        return "bg-red-900 text-red-100 active:bg-red-700 active:text-red-300 dark:bg-red-100 dark:text-red-900 dark:hover:bg-red-200 dark:hover:text-red-800 dark:active:bg-red-300 dark:active:text-red-700 hover:bg-red-800 hover:text-red-200";
      }
    }
  };
</script>

<button
  {...$$props}
  class={`
    flex flex-col items-center p-2 shadow transition-all

    ${fullyRounded ? "rounded-full" : "rounded"}
    ${getColorClasses()}
    ${$$props.class}
  `}
  on:click
>
  <div class="flex items-center">
    <slot />
  </div>
</button>
