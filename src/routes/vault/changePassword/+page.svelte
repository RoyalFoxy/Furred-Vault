<script lang="ts">
  import { invoke } from "@tauri-apps/api";

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

  const changeVaultPassword = async (e: Event) => {
    e.preventDefault();

    globalLoadingSpinner.on();

    try {
      await invoke("change_password_of_vault", {
        vault: $selectedVault,
        oldPassword,
        newPassword,
      });

      $enteredPassword = newPassword;

      $vaultData = await invoke("load_vault", {
        vault: $selectedVault,
        password: $enteredPassword,
      });

      toasts.push({
        message: `Password of vault **${$selectedVault}** was changed`,
        type: "success",
      });

      await goto(`/vault/view`);
    } catch (e) {
      pushError(e);
    } finally {
      globalLoadingSpinner.off();
    }
  };

  const cancelChange = async () => {
    await goto("/vault/view");
  };

  $: if ($closePopover) {
    resetClosePopover();
    goto("/vault/view");
  }

  let oldPassword = "";
  let newPassword = "";
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
    <form class="flex flex-col text-lg" on:submit={changeVaultPassword}>
      <h2 class="text-center text-2xl">Vault</h2>

      <div class="my-2">
        <Input
          disabled
          label="Vault"
          placeholder="Vault"
          required
          type="text"
          bind:value={$selectedVault}
        />
      </div>

      <div class="mb-2">
        <PasswordInput
          label="Old password"
          showPasswordStrength
          visibilityToggle
          bind:password={oldPassword}
        />
      </div>

      <div class="mb-2">
        <PasswordInput
          generatePassword
          label="New password"
          showPasswordStrength
          visibilityToggle
          bind:password={newPassword}
        />
      </div>

      <div class="grid grid-cols-2 gap-2">
        <Button style="success" type="submit">
          Change <FilledIcon class="ml-1" icon="LockReset" />
        </Button>
        <Button type="button" on:click={cancelChange}>
          Cancel <FilledIcon class="ml-1" icon="Close" />
        </Button>
      </div>
    </form>
  </div>
</div>
