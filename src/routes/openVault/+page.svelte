<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import type { VaultData } from "$lib/types/generated/VaultData";

  import { goto } from "$app/navigation";
  import Button from "$lib/components/Button.svelte";
  import Input from "$lib/components/Input.svelte";
  import PasswordInput from "$lib/components/PasswordInput.svelte";
  import FilledIcon from "$lib/components/icons/FilledIcon.svelte";
  import { closePopover, resetClosePopover } from "$lib/stores/closePopover";
  import { enteredPassword } from "$lib/stores/enteredPassword";
  import { globalLoadingSpinner } from "$lib/stores/globalLoadingSpinner";
  import { selectedVault } from "$lib/stores/selectedVault";
  import { pushError, toasts } from "$lib/stores/toasts";
  import { vaultData } from "$lib/stores/vaultData";

  if (!$selectedVault) {
    goto("/");
  }

  const openVault = async (e: Event) => {
    e.preventDefault();

    globalLoadingSpinner.on();

    try {
      let data = await invoke<VaultData>("load_vault", {
        vault: $selectedVault,
        password: $enteredPassword,
      });

      vaultData.set(data);

      toasts.push({ message: `Vault **${$selectedVault}** was unlocked`, type: "info" });

      await goto("/vault/view");
    } catch (e) {
      pushError(e);
    } finally {
      globalLoadingSpinner.off();
    }
  };

  const closeVaultOpenForm = async () => {
    $selectedVault = null;
    resetClosePopover();
    await goto("/");
  };

  $enteredPassword = null;

  $: if ($closePopover) {
    $selectedVault = null;
    resetClosePopover();
    goto("/");
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
    <form class="flex flex-col text-lg" on:submit={openVault}>
      <h2 class="text-center text-2xl">Open Vault</h2>

      <div class="mt-2">
        <Input
          disabled
          label="Vault"
          placeholder="Vault name"
          required
          type="text"
          bind:value={$selectedVault}
        />

        <div class="my-2">
          <PasswordInput
            label="Password"
            shouldFocus
            visibilityToggle
            bind:password={$enteredPassword}
          />
        </div>

        <div class="grid w-full grid-cols-2 gap-2">
          <Button style="success" type="submit">
            Unlock <FilledIcon class="ml-1" icon="LockOpen" />
          </Button>
          <Button type="button" on:click={closeVaultOpenForm}>
            Cancel <FilledIcon class="ml-1" icon="Close" />
          </Button>
        </div>
      </div>
    </form>
  </div>
</div>
