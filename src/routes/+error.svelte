<script lang="ts">
  import { page } from "$app/stores";

  console.log({
    status: $page.status,
    message: $page.error?.message,
  });
</script>

<h1>{$page.status}: {$page.error?.message}</h1>
<a href="/">Home</a>
