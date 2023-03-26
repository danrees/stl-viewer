<script lang="ts">
  import { onMount } from "svelte";
  import { createScene } from "./lib/scene";
  import { readBinaryFile } from "@tauri-apps/api/fs";
  import { invoke } from "@tauri-apps/api/tauri";

  let el: HTMLCanvasElement;
  const loadSTL = async (path: string) => {
    const stl: Uint8Array = await invoke("load_stl", { name: path });
    const buffer = new ArrayBuffer(stl.length);
    const newSTL = new Uint8Array(buffer);
    let something = false;
    stl.forEach((val, i) => {
      if (!something) {
        something = true;
      }
      newSTL[i] = val;
    });
    console.log(something);
    console.log(stl.length);
    console.log(newSTL.length);
    return newSTL;
  };
  const fileLoc = "";

  onMount(async () => {
    const data = await loadSTL(fileLoc);
    createScene(el, window, data.buffer);
  });
</script>

<canvas bind:this={el} />
