<script lang="ts">
  import { onMount } from "svelte";
  import { createScene } from "./lib/scene";

  let el: HTMLCanvasElement;

  onMount(() => {
    createScene(el, window);
  });
</script>

<canvas bind:this={el} />
