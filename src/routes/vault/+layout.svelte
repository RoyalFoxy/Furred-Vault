<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { confirm } from "@tauri-apps/api/dialog";
  import { flip } from "svelte/animate";
  import { fade } from "svelte/transition";

  import type { PasswordEntry } from "$lib/types/generated/PasswordEntry";

  import { beforeNavigate, goto } from "$app/navigation";
  import Button from "$lib/components/Button.svelte";
  import Input from "$lib/components/Input.svelte";
  import FilledIcon from "$lib/components/icons/FilledIcon.svelte";
  import { enteredPassword } from "$lib/stores/enteredPassword";
  import { globalLoadingSpinner } from "$lib/stores/globalLoadingSpinner";
  import { selectedVault } from "$lib/stores/selectedVault";
  import { pushError, toasts } from "$lib/stores/toasts";
  import { vaultData } from "$lib/stores/vaultData";
  import { vaults } from "$lib/stores/vaults";

  if (!$vaultData) {
    goto("/");
  }

  const currentVault = $selectedVault;

  beforeNavigate(({ to, from }) => {
    if (
      /^\/vault\/?.*/.test(from?.url.pathname || "") &&
      /^\/(?!vault)\/?.*/.test(to?.url.pathname || "")
    ) {
      if (currentVault === $selectedVault) {
        toasts.push({ message: `Vault **${$selectedVault}** was locked`, type: "info" });
        $selectedVault = null;
      }
      $enteredPassword = null;
      $vaultData = null;
    }
  });

  const lockVault = async () => {
    await goto("/");
  };

  const changePasswordOfVault = async () => {
    await goto("/vault/changePassword");
  };

  const renameVault = async () => {
    await goto("/vault/rename");
  };

  const deleteVault = async () => {
    try {
      const confirmed = await confirm("This action cannot be reverted. Are you sure?", {
        okLabel: `Delete ${$selectedVault}`,
        cancelLabel: "Keep",
        title: "Furred Vault",
        type: "warning",
      });

      globalLoadingSpinner.on();

      if (!confirmed) {
        return;
      }

      await invoke("delete_vault", { vault: $selectedVault });

      $vaults = await invoke("get_vaults");

      const deletedVault = $selectedVault;
      $selectedVault = null;

      toasts.push({ message: `Vault **${deletedVault}** was deleted`, type: "info" });

      await goto("/");
    } catch (e) {
      pushError(e);
    } finally {
      globalLoadingSpinner.off();
    }
  };

  const createPassword = async () => {
    await goto("/vault/createPassword");
  };

  const openPasswordEntry = async (passwordId: string) => {
    await goto(`/vault/password?passwordId=${passwordId}`);
  };

  let filteredPasswords: PasswordEntry[] = [];
  $: $vaultData && filterPassword(searchTerm, $vaultData.passwords);

  $: uniqueTags = filteredPasswords.reduce((tags, password) => {
    for (const passwordTag of password.tags) {
      if (!tags.includes(passwordTag)) {
        tags.push(passwordTag);
      }
    }

    return tags;
  }, [] as string[]);

  let selectedTag: string = "NONE";

  const filterPassword = async (searchTerm: string, passwords: PasswordEntry[]) => {
    if (!searchTerm) {
      return (filteredPasswords = passwords || []);
    }

    try {
      filteredPasswords = (
        await invoke<[number, string][]>("search_passwords", {
          passwords,
          searchTerm,
        })
      )
        .filter(([score]) => score > 0)
        .toSorted(([aScore], [bScore]) => bScore - aScore)
        .map(([, uuid]) => $vaultData!.passwords.find(({ id }) => id === uuid)!);

      console.log(filteredPasswords);
    } catch (e) {
      pushError(e);
    } finally {
      globalLoadingSpinner.off();
    }
  };

  let searchTerm = "";

  const pageSize = 50;

  let maxPage = 0;

  $: maxPage = Math.floor(filteredPasswords.length / pageSize);
  let page = 0;

  const nextPage = () => {
    page++;
  };

  const prevPage = () => {
    page--;
  };

  document.onkeydown = (e) => {
    if (e.code === "ArrowRight") {
      nextPage();
    }

    if (e.code === "ArrowLeft") {
      prevPage();
    }
  };

  $: if (page > maxPage) {
    page = maxPage;
  }
  $: if (page < 0) {
    page = 0;
  }
  $: previousDisabled = page === 0;
  $: nextDisabled = page === maxPage;

  $: paginatedPasswords = filteredPasswords
    .filter(({ tags }) =>
      selectedTag === "NONE" ? true : tags.includes(selectedTag.replace("tag-value-", "")),
    )
    .slice(page * pageSize, (page + 1) * pageSize);
</script>

<div class="relative flex flex-col items-center">
  <div class="absolute right-0 flex h-screen flex-col justify-end gap-2 p-2">
    <Button class="size-12 !p-1.5" fullyRounded on:click={lockVault}>
      <FilledIcon class="size-full" icon="Lock" />
    </Button>

    <Button class="size-12 !p-1.5" fullyRounded on:click={changePasswordOfVault}>
      <FilledIcon class="size-full" icon="LockReset" />
    </Button>

    <Button class="size-12 !p-1.5" fullyRounded on:click={renameVault}>
      <FilledIcon class="size-full" icon="Edit" />
    </Button>

    <Button style="destructive" class="size-12 !p-1.5" fullyRounded on:click={deleteVault}>
      <FilledIcon class="size-full" icon="Delete" />
    </Button>

    <Button style="success" class="size-12 !p-1" fullyRounded on:click={createPassword}>
      <FilledIcon class="size-full" icon="Add" />
    </Button>
  </div>

  <h1
    class="
      m-4 overflow-hidden text-ellipsis text-nowrap text-center text-xl
      font-bold

      lg:text-2xl
    "
  >
    Passwords - {$selectedVault}
  </h1>

  <div class="flex gap-2">
    <Input placeholder="Search" spellcheck="false" wrapperClass="grow" bind:value={searchTerm} />

    <select
      class="
        grow resize-none appearance-none rounded bg-zinc-100 p-2 transition-all

        dark:bg-zinc-900

        disabled:text-zinc-500
      "
      bind:value={selectedTag}
    >
      <option value="NONE">-</option>
      {#each uniqueTags as tag}
        <option value={`tag-value-${tag}`}>{tag}</option>
      {/each}
    </select>
  </div>

  <div class="mt-2 flex">
    <button disabled={previousDisabled} type="button" on:click={prevPage}>
      <FilledIcon class={previousDisabled && "text-zinc-500"} icon="NavigateBefore" />
    </button>

    <p>{page + 1} / {maxPage + 1}</p>

    <button disabled={nextDisabled} type="button" on:click={nextPage}>
      <FilledIcon class={nextDisabled && "text-zinc-500"} icon="NavigateNext" />
    </button>
  </div>

  <div
    class="
      flex h-screen w-[95%] flex-col

      2xl:w-[50%]

      lg:w-[80%]

      md:w-[90%]

      xl:w-[65%]
    "
  >
    {#if $vaultData}
      {#if filteredPasswords.length}
        {#key page}
          <div
            class="
              mb-2 w-full flex-1 divide-y divide-zinc-900/25 overflow-scroll
              rounded p-2 shadow scrollbar-hide

              dark:divide-zinc-100/25
            "
            in:fade={{ duration: 150 }}
          >
            {#each paginatedPasswords as password (password.website)}
              <div
                class="w-full cursor-pointer p-2"
                transition:fade={{ duration: 150 }}
                animate:flip={{ duration: 150 }}
              >
                <button class="w-full text-left" on:click={() => openPasswordEntry(password.id)}>
                  <p class="overflow-hidden text-ellipsis text-nowrap">{password.website}</p>
                </button>
              </div>
            {/each}
          </div>
        {/key}
      {:else}
        <div class="flex h-screen w-full items-center justify-center">
          <h1 class="text-2xl">No passwords saved yet :c</h1>
        </div>
      {/if}
    {:else}
      <p>Something went wrong</p>
    {/if}

    <slot />
  </div>
</div>
