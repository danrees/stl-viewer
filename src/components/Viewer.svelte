<script lang="ts">
  import { onMount } from "svelte";
  import { createScene } from "../lib/scene";
  import { invoke } from "@tauri-apps/api/tauri";

  let el: HTMLCanvasElement;
  const loadSTL = async (id: string) => {
    console.log(id);
    const stl: Uint8Array = await invoke("load_stl", { id: id });
    const buffer = new ArrayBuffer(stl.length);
    const newSTL = new Uint8Array(buffer);
    let something = false;
    stl.forEach((val, i) => {
      if (!something) {
        something = true;
      }
      newSTL[i] = val;
    });

    return newSTL;
  };
  //export let stlPath: string;
  export let params: { id: string };

  onMount(async () => {
    const data = await loadSTL(params.id);
    const animate = createScene(el, window, data.buffer);
    animate();
  });
</script>

<canvas bind:this={el} />
