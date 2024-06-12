<script lang="ts">
  import { writeText } from "@tauri-apps/api/clipboard";

  import FilledIcon from "./icons/FilledIcon.svelte";

  export let content: string | null;
</script>

<button type="button" on:click={async () => content && (await writeText(content))}>
  <FilledIcon icon="CopyContent" />
</button>
