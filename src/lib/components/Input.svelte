<script lang="ts">
  export let area: boolean = false;
  export let label: string | undefined = undefined;
  export let disabled: boolean = false;

  export let value: string | null = null;

  export let wrapperClass: string | null = null;

  if (!$$props.id && label) {
    $$props.id = `${label?.toLowerCase()}-${area ? "text-area" : "input"}`;
  }

  export let inputElement: HTMLInputElement | HTMLTextAreaElement | null = null;
</script>

{#if label}
  <label class="pl-1 text-sm" for={$$props.id}>
    {label}
    {#if $$props.required}
      <span class="text-red-500">*</span>
    {/if}
  </label>
{/if}

<div
  class={`
    flex gap-2

    ${wrapperClass}
  `}
>
  {#if area}
    <textarea
      {...$$props}
      bind:this={inputElement}
      class={`
        grow resize-none rounded bg-zinc-100 p-2 transition-all

        dark:bg-zinc-900

        disabled:text-zinc-500

        ${$$props.class}
      `}
      {disabled}
      on:change
      bind:value
    />
  {:else}
    <input
      {...$$props}
      bind:this={inputElement}
      class={`
        grow resize-none rounded bg-zinc-100 p-2 transition-all

        dark:bg-zinc-900

        disabled:text-zinc-500

        ${$$props.class}
      `}
      {disabled}
      on:change
      bind:value
    />
  {/if}
  <slot />
</div>
