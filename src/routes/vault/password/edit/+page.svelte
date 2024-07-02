<script lang="ts">
  import { invoke } from "@tauri-apps/api";

  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
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

  let passwordId = $page.url.searchParams.get("passwordId")!;

  let selectedPassword = structuredClone(
    $vaultData?.passwords.find(({ id }) => id === passwordId) || null,
  );

  if (!selectedPassword) {
    goto("/vault/view");
  }

  const cancelEdit = async () => {
    await goto(`/vault/password?passwordId=${passwordId}`);
  };

  const editSelectedPassword = async (e: Event) => {
    e.preventDefault();

    globalLoadingSpinner.on();

    try {
      await invoke("update_password_entry", {
        vault: $selectedVault,
        password: $enteredPassword,
        entryToUpdate: selectedPassword,
      });

      $vaultData = await invoke("load_vault", {
        vault: $selectedVault,
        password: $enteredPassword,
      });

      toasts.push({
        message: `Password **${selectedPassword?.website}** was modified`,
        type: "info",
      });

      await goto(`/vault/password?passwordId=${passwordId}`);
    } catch (e) {
      pushError(e);
    } finally {
      globalLoadingSpinner.off();
    }
  };

  const onTagChange = ({ target }: Event) => {
    selectedPassword!.tags = ((target as HTMLInputElement).value || "")
      .split(",")
      .map((tag) => tag.trim());
  };

  $: if ($closePopover) {
    resetClosePopover();
    goto("/vault/view");
  }
</script>

{#if selectedPassword}
  <div
    class="
      pointer-events-none absolute left-0 top-0 flex h-screen w-full
      items-center justify-center
    "
  >
    <div
      class="
        pointer-events-auto w-96 rounded bg-zinc-50/25 p-2 shadow backdrop-blur
        transition-all

        dark:bg-zinc-950/25
      "
    >
      <form class="flex flex-col text-lg" on:submit={editSelectedPassword}>
        <h2 class="text-center text-2xl">Password entry</h2>

        <div class="mb-2 flex flex-col">
          <Input
            id="username-input"
            label="Username"
            placeholder="Username"
            required
            type="text"
            bind:value={selectedPassword.username}
          />
        </div>

        <div class="mb-2 flex flex-col">
          <PasswordInput
            generatePassword
            label="password"
            showPasswordStrength
            visibilityToggle
            bind:password={selectedPassword.password}
          />
        </div>

        <div class="mb-2 flex flex-col">
          <Input
            id="website-input"
            label="Website"
            placeholder="No website added"
            type="text"
            bind:value={selectedPassword.website}
          />
        </div>

        <Input
          class="mb-2"
          label="Tags"
          placeholder="Tags"
          type="text"
          value={selectedPassword.tags.join(", ")}
          on:change={onTagChange}
        />

        <div class="mb-2 flex flex-col">
          <Input
            id="notes-input"
            area
            label="Notes"
            placeholder="No notes added"
            rows="4"
            bind:value={selectedPassword.notes}
          />
        </div>

        <div class="grid grid-cols-2 gap-2">
          <Button type="submit">
            Save <FilledIcon class="ml-1" icon="Edit" />
          </Button>
          <Button type="button" on:click={cancelEdit}>
            Cancel <FilledIcon class="ml-1" icon="Close" />
          </Button>
        </div>
      </form>
    </div>
  </div>
{/if}
