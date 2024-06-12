<script lang="ts">
  import { invoke } from "@tauri-apps/api";

  import FilledIcon from "./icons/FilledIcon.svelte";

  export let password;

  const generatePassword = async () => {
    password = await invoke("suggest_strong_password");
  };
</script>

<button type="button" on:click={generatePassword}>
  <FilledIcon icon="Password" />
</button>
