<script lang="ts">
  import CopyButton from "$lib/components//CopyButton.svelte";
  import GeneratePassword from "$lib/components/GeneratePassword.svelte";
  import Input from "$lib/components/Input.svelte";
  import PasswordStrength from "$lib/components/PasswordStrength.svelte";
  import PasswordVisibilityToggle from "$lib/components/PasswordVisibilityToggle.svelte";

  export let password: string | null;
  export let label: string = "";

  export let disabled: boolean = false;
  export let visibilityToggle: boolean = false;
  export let generatePassword: boolean = false;
  export let shouldFocus: boolean = false;
  export let canCopy: boolean = false;
  export let showPasswordStrength: boolean = false;

  let passwordHidden = true;
  let passwordInput: HTMLInputElement | null = null;

  $: shouldFocus && passwordInput && passwordInput.focus();
</script>

<div class="relative">
  <Input
    {disabled}
    placeholder="Password"
    required
    type={passwordHidden ? "password" : "text"}
    bind:label
    bind:value={password}
    bind:inputElement={passwordInput}
  >
    {#if visibilityToggle}
      <PasswordVisibilityToggle bind:passwordHidden />
    {/if}
    {#if canCopy}
      <CopyButton bind:content={password} />
    {/if}
    {#if generatePassword}
      <GeneratePassword bind:password />
    {/if}
    {#if showPasswordStrength}
      <PasswordStrength bind:password />
    {/if}
  </Input>
</div>
