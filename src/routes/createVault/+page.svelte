<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import { goto } from "$app/navigation";
  import Button from "$lib/components/Button.svelte";
  import Input from "$lib/components/Input.svelte";
  import PasswordInput from "$lib/components/PasswordInput.svelte";
  import FilledIcon from "$lib/components/icons/FilledIcon.svelte";
  import { closePopover, resetClosePopover } from "$lib/stores/closePopover";
  import { globalLoadingSpinner } from "$lib/stores/globalLoadingSpinner";
  import { pushError, toasts } from "$lib/stores/toasts";
  import { vaults } from "$lib/stores/vaults";

  const createVault = async (e: Event) => {
    e.preventDefault();

    globalLoadingSpinner.on();
    try {
      await invoke("create_vault", input);
      vaults.set(await invoke<string[]>("get_vaults"));

      toasts.push({ message: `Vault **${input.vault}** was created`, type: "success" });

      input = structuredClone(defaultInput);
      await goto("/");
    } catch (e) {
      pushError(e);
    } finally {
      globalLoadingSpinner.off();
    }
  };

  const closeCreationForm = async () => {
    input = structuredClone(defaultInput);

    await goto("/");
  };

  const defaultInput = {
    vault: "",
    password: "",
  };

  let input = structuredClone(defaultInput);

  let vaultInput: HTMLInputElement | null;

  $: vaultInput && vaultInput.focus();

  $: if ($closePopover) {
    resetClosePopover();
    goto("/");
  }
</script>

<div
  class="
    pointer-events-none absolute left-0 top-0 flex size-full items-center
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
    <form class="flex flex-col text-lg" on:submit={createVault}>
      <h2 class="text-center text-2xl">Create Vault</h2>

      <div class="mt-2">
        <Input
          label="Vault"
          placeholder="Vault name"
          required
          type="text"
          bind:value={input.vault}
          bind:inputElement={vaultInput}
        />
      </div>

      <div class="my-2">
        <PasswordInput
          generatePassword
          label="Password"
          showPasswordStrength
          visibilityToggle
          bind:password={input.password}
        />
      </div>

      <div class="grid w-full grid-cols-2 gap-2">
        <Button style="success" type="submit">
          Create <FilledIcon class="ml-1" icon="Add" />
        </Button>
        <Button type="button" on:click={closeCreationForm}>
          Cancel <FilledIcon class="ml-1" icon="Close" />
        </Button>
      </div>
    </form>
  </div>
</div>
