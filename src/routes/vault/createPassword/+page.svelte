<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import type { PasswordEntry } from "$lib/types/generated/PasswordEntry";

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

  const createPassword = async (e: Event) => {
    e.preventDefault();

    globalLoadingSpinner.on();

    try {
      const uuid = await invoke<string>("get_uuid");
      input.id = uuid;

      await invoke("create_password_entry", {
        vault: $selectedVault,
        password: $enteredPassword,
        entry: input,
      });

      $vaultData = await invoke("load_vault", {
        vault: $selectedVault,
        password: $enteredPassword,
      });

      toasts.push({ message: `Created password in **${$selectedVault}**`, type: "info" });

      await goto("/vault/view");
    } catch (e) {
      pushError(e);
    } finally {
      globalLoadingSpinner.off();
    }
  };

  const closeCreationForm = async () => {
    await goto("/vault/view");
  };

  const onTagChange = ({ target }: Event) => {
    input.tags = ((target as HTMLInputElement).value || "").split(",").map((tag) => tag.trim());
  };

  const input: PasswordEntry = {
    id: "",
    username: "",
    password: "",
    notes: "",
    website: "",
    tags: [],
  };

  let usernameInput: HTMLInputElement | null;

  $: usernameInput && usernameInput.focus();

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
    <form class="flex flex-col text-lg" on:submit={createPassword}>
      <h2 class="text-center text-2xl">Create Password</h2>

      <Input
        class="my-2"
        label="Username"
        placeholder="Username"
        required
        type="text"
        bind:value={input.username}
        bind:inputElement={usernameInput}
      />

      <div class="mb-2 flex flex-col">
        <PasswordInput
          generatePassword
          label="Password"
          showPasswordStrength
          visibilityToggle
          bind:password={input.password}
        />
      </div>

      <Input
        class="mb-2"
        label="Website or Identifier"
        placeholder="https://example.com"
        required
        spellcheck="false"
        bind:value={input.website}
      />

      <Input class="mb-2" label="Tags" placeholder="Tags" type="text" on:change={onTagChange} />

      <Input
        class="mb-2"
        area
        label="Notes"
        placeholder="Notes"
        rows="4"
        type="text"
        bind:value={input.notes}
      />

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
