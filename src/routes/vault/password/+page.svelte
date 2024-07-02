<script lang="ts" strictEvents>
  import { confirm } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api/tauri";

  import type { PasswordEntry } from "$lib/types/generated/PasswordEntry";

  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import Button from "$lib/components/Button.svelte";
  import CopyButton from "$lib/components/CopyButton.svelte";
  import Input from "$lib/components/Input.svelte";
  import PasswordInput from "$lib/components/PasswordInput.svelte";
  import FilledIcon from "$lib/components/icons/FilledIcon.svelte";
  import { closePopover, resetClosePopover } from "$lib/stores/closePopover";
  import { enteredPassword } from "$lib/stores/enteredPassword";
  import { globalLoadingSpinner } from "$lib/stores/globalLoadingSpinner";
  import { selectedVault } from "$lib/stores/selectedVault";
  import { pushError, toasts } from "$lib/stores/toasts";
  import { vaultData } from "$lib/stores/vaultData";

  let selectedPassword: PasswordEntry | null = null;

  const getPasswordData = (passwordId: string) => {
    const selectedPassword = $vaultData?.passwords.find(({ id }) => id === passwordId) || null;

    if (!selectedPassword) {
      goto("/vault/view");
    }

    return selectedPassword;
  };

  $: passwordId = $page.url.searchParams.get("passwordId")!;
  $: selectedPassword = getPasswordData(passwordId);

  const deleteSelectedPassword = async (event: MouseEvent) => {
    try {
      const confirmed =
        event.shiftKey ||
        (await confirm("This action cannot be reverted. Are you sure?", {
          okLabel: "Delete",
          cancelLabel: "Keep",
          title: "Furred Vault",
          type: "warning",
        }));

      globalLoadingSpinner.on();

      if (!confirmed) {
        return;
      }

      await invoke("delete_password_entry", {
        vault: $selectedVault,
        password: $enteredPassword,
        entryIdToDelete: passwordId,
      });

      $vaultData = await invoke("load_vault", {
        vault: $selectedVault,
        password: $enteredPassword,
      });

      toasts.push({
        message: `Password **${selectedPassword?.website}** was deleted`,
        type: "info",
      });

      await goto("/vault/view");
    } catch (e) {
      pushError(e);
    } finally {
      globalLoadingSpinner.off();
    }
  };

  const closeSelectedPassword = async () => {
    selectedPassword = null;
    await goto("/vault/view");
  };

  const editSelectedPassword = async (e: Event) => {
    e.preventDefault();

    await goto(`/vault/password/edit?passwordId=${passwordId}`);
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
            disabled
            label="Username"
            placeholder="Username"
            required
            type="text"
            value={selectedPassword.username}
          >
            <CopyButton bind:content={selectedPassword.username} />
          </Input>
        </div>

        <div class="mb-2 flex flex-col">
          <PasswordInput
            canCopy
            disabled={true}
            label="password"
            showPasswordStrength
            visibilityToggle
            bind:password={selectedPassword.password}
          />
        </div>

        <div class="mb-2 flex flex-col">
          <Input
            id="website-input"
            disabled
            label="Website"
            placeholder="No website added"
            type="text"
            value={selectedPassword.website}
          />
        </div>

        <Input
          class="mb-2"
          disabled
          label="Tags"
          placeholder="Tags"
          type="text"
          value={selectedPassword.tags.join(", ")}
        />

        <div class="mb-2 flex flex-col">
          <Input
            id="notes-input"
            area
            disabled
            label="Notes"
            placeholder="No notes added"
            rows="4"
            value={selectedPassword.notes}
          />
        </div>

        <div class="grid grid-cols-3 gap-2">
          <Button type="submit">
            Edit <FilledIcon class="ml-1" icon="Edit" />
          </Button>
          <Button style="destructive" type="button" on:click={deleteSelectedPassword}>
            Delete <FilledIcon class="ml-1" icon="Delete" />
          </Button>
          <Button type="button" on:click={closeSelectedPassword}>
            Close <FilledIcon class="ml-1" icon="Close" />
          </Button>
        </div>
      </form>
    </div>
  </div>
{/if}
