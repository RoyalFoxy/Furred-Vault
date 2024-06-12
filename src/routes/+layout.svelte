<script lang="ts">
  import "../app.css";
  import { invoke } from "@tauri-apps/api/tauri";
  import { flip } from "svelte/animate";
  import { fly } from "svelte/transition";

  import { goto } from "$app/navigation";
  import Button from "$lib/components/Button.svelte";
  import Cursor from "$lib/components/Cursor.svelte";
  import GlobalLoadingSpinner from "$lib/components/GlobalLoadingSpinner.svelte";
  import Throbber from "$lib/components/Throbber.svelte";
  import Toasts from "$lib/components/Toasts.svelte";
  import { LAYOUT_CONTAINER } from "$lib/constants";
  import { globalKeyStore } from "$lib/stores/globalKeyStore";
  import { globalMouseStore } from "$lib/stores/globalMouseStore";
  import { selectedVault } from "$lib/stores/selectedVault";
  import { pushError } from "$lib/stores/toasts";
  import { vaults } from "$lib/stores/vaults";

  invoke<string[]>("get_vaults").then(vaults.set).catch(pushError);

  const openVaultCreationForm = async () => {
    await goto("/createVault");
  };

  const openVaultOpenForm = async (vault: string) => {
    $selectedVault = vault;

    goto("/openVault");
  };

  window.onclick = (event) => ($globalMouseStore = { event, emitTime: new Date() });
  window.onkeydown = (event) => ($globalKeyStore = { event, emitTime: new Date() });

  setTimeout(() => {
    invoke("close_splashscreen").catch(pushError);
  }, 1500);
</script>

<GlobalLoadingSpinner />
<Toasts />
<Cursor />

<main
  class="
    flex h-screen w-screen overflow-hidden bg-zinc-50 text-zinc-950

    dark:bg-zinc-950 dark:text-zinc-50
  "
>
  <div class="flex h-screen w-48 flex-col shadow">
    <h1 class="p-2 text-center text-2xl">Vaults</h1>
    <div class="flex min-h-0 w-full grow flex-col items-center">
      {#if $vaults === null}
        <Throbber />
      {:else if $vaults.length}
        <div
          class="
            h-full w-full divide-y divide-zinc-900/25 overflow-scroll
            scrollbar-hide

            dark:divide-zinc-100/25
          "
        >
          {#each $vaults as vault (vault)}
            <button
              class="w-48 overflow-hidden text-ellipsis p-2"
              on:click={() => openVaultOpenForm(vault)}
              in:fly={{ x: -200 }}
              out:fly={{ x: -200 }}
              animate:flip
            >
              {#if vault === $selectedVault}
                <span class="font-bold">{vault}</span>
              {:else}
                {vault}
              {/if}
            </button>
          {/each}
        </div>
      {:else}
        <p class="w-full text-center">No vaults yet :c</p>
      {/if}
    </div>

    <Button class="m-2" on:click={openVaultCreationForm}>Create Vault</Button>
  </div>

  <div id={LAYOUT_CONTAINER} class="relative min-h-0 min-w-0 grow">
    <slot />
  </div>
</main>
