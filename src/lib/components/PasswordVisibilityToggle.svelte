<script lang="ts">
  import FilledIcon from "./icons/FilledIcon.svelte";

  const togglePasswordVisibility = () => {
    passwordHidden = !passwordHidden;
  };

  export let passwordHidden: boolean;
</script>

<button type="button" on:click={togglePasswordVisibility}>
  {#if passwordHidden}
    <FilledIcon icon="Visibility" />
  {:else}
    <FilledIcon icon="VisibilityOff" />
  {/if}
</button>
