<script lang="ts">
  import { cubicOut } from "svelte/easing";
  import { tweened } from "svelte/motion";

  const X = tweened(0, { duration: 250, easing: cubicOut });
  const Y = tweened(0, { duration: 350, easing: cubicOut });

  const size = tweened(12, { duration: 250, easing: cubicOut });

  let cursor = "none";

  let smallBackdrop = false;

  window.onmousemove = (event) => {
    cursor = window.getComputedStyle(event.target as Element).cursor;

    switch (cursor) {
      case "pointer": {
        $size = 16;
        smallBackdrop = true;
        break;
      }

      case "text": {
        $size = 8;
        smallBackdrop = true;
        break;
      }

      default: {
        smallBackdrop = false;
        $size = 12;
        break;
      }
    }

    $X = event.clientX;
    $Y = event.clientY;
  };
</script>

<div
  style:width={`${$size / 4}rem`}
  style:height={`${$size / 4}rem`}
  style:transform={`translate3d(calc(${$X}px - 50%), calc(${$Y}px - 50%), 0)`}
  class={`
    pointer-events-none fixed z-50 size-12 rounded-full border-2
    border-zinc-950/10 bg-zinc-950/10 will-change-transform contain-strict

    ${smallBackdrop ? "backdrop-blur-sm" : "backdrop-blur"}

    dark:border-zinc-50/10 dark:bg-zinc-50/10
  `}
/>
