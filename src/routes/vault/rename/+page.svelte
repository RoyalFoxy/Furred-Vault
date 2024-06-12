<script lang="ts">
  import { invoke } from "@tauri-apps/api";

  import { goto } from "$app/navigation";
  import Button from "$lib/components/Button.svelte";
  import Input from "$lib/components/Input.svelte";
  import FilledIcon from "$lib/components/icons/FilledIcon.svelte";
  import { closePopover, resetClosePopover } from "$lib/stores/closePopover";
  import { enteredPassword } from "$lib/stores/enteredPassword";
  import { globalLoadingSpinner } from "$lib/stores/globalLoadingSpinner";
  import { selectedVault } from "$lib/stores/selectedVault";
  import { pushError, toasts } from "$lib/stores/toasts";
  import { vaultData } from "$lib/stores/vaultData";
  import { vaults } from "$lib/stores/vaults";

  let name = $selectedVault;

  const renameVault = async (e: Event) => {
    e.preventDefault();

    globalLoadingSpinner.on();

    try {
      await invoke("rename_vault", {
        oldVault: $selectedVault,
        newVault: name,
        password: $enteredPassword,
      });

      const oldName = $selectedVault;
      $selectedVault = name;

      $vaultData = await invoke("load_vault", {
        vault: $selectedVault,
        password: $enteredPassword,
      });

      $vaults = await invoke("get_vaults");

      toasts.push({
        message: `Vault **${oldName}** was renamed to **${$selectedVault}**`,
        type: "info",
      });

      await goto(`/vault/view`);
    } catch (e) {
      pushError(e);
    } finally {
      globalLoadingSpinner.off();
    }
  };

  const cancelRename = async () => {
    await goto("/vault/view");
  };

  $: if ($closePopover) {
    resetClosePopover();
    goto("/vault/view");
  }
</script>

<div
  class="
    pointer-events-none absolute left-0 top-0 flex h-screen w-full items-center
    justify-center
  "
>
  <div
    class="
      pointer-events-auto w-96 rounded bg-zinc-50/25 p-2 shadow backdrop-blur
      transition-all

      dark:bg-zinc-950/25
    "
  >
    <form class="flex flex-col text-lg" on:submit={renameVault}>
      <h2 class="text-center text-2xl">Vault</h2>

      <div class="mb-2 flex flex-col">
        <Input label="Vault" placeholder="Vault" required type="text" bind:value={name} />
      </div>

      <div class="grid grid-cols-2 gap-2">
        <Button type="submit">
          Rename <FilledIcon class="ml-1" icon="Edit" />
        </Button>
        <Button type="button" on:click={cancelRename}>
          Cancel <FilledIcon class="ml-1" icon="Close" />
        </Button>
      </div>
    </form>
  </div>
</div>
