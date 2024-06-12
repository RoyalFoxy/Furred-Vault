<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { cubicOut } from "svelte/easing";
  import { tweened } from "svelte/motion";

  import { pushError } from "$lib/stores/toasts";

  export let password;

  let strength = tweened(0, { duration: 350, easing: cubicOut });

  $: if (password) {
    invoke<number>("analyze_password", { password })
      .then((result) => ($strength = result))
      .catch(pushError);
  } else {
    $strength = 0;
  }

  const size = 36;
  const offset = size / 2;
  const radius = offset * 0.9;
  const circumference = radius * Math.PI * 2;
  const strokeWidth = size / 20;

  let color = "text-red-500";

  $: if ($strength < 40) {
    color = "text-red-500";
  } else if ($strength < 60) {
    color = "text-orange-500";
  } else if ($strength < 80) {
    color = "text-yellow-500";
  } else if ($strength < 95) {
    color = "text-green-300";
  } else {
    color = "text-green-500";
  }
</script>

<svg class="group self-center" height={size} viewBox={`0 0 ${size} ${size}`} width={size}>
  <circle class="fill-zinc-900" cx={offset} cy={offset} r={radius} stroke-width={strokeWidth} />
  <circle
    class={`
      ${color}
    `}
    cx={offset}
    cy={offset}
    fill="none"
    r={radius}
    stroke="currentColor"
    stroke-dasharray={circumference}
    stroke-dashoffset={circumference * (1 - $strength / 100)}
    stroke-width={strokeWidth * 2}
    transform={`rotate(135 ${offset} ${offset})`}
  />
</svg>
