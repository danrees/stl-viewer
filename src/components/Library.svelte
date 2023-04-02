<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from "@tauri-apps/api/dialog";
  import { onMount } from "svelte";
  import type { Library } from "../lib/file";

  let library: string;
  let name: string;
  let libraries: Library[] = [];
  const click = async () => {
    try {
      const lib = await invoke("save_library", { name: name, path: library });
      reload();
    } catch (e) {
      alert(e);
    }
  };
  const reload = async () => {
    try {
      const l: Library[] = await invoke("list_libraries");
      console.log(l);
      libraries = l;
    } catch (e) {
      alert(e);
    }
  };
  const load = async () => {
    const resp = await open({ directory: true, multiple: false });

    if (Array.isArray(resp)) {
      return;
    }
    library = resp;
  };

  const deleteLib = async (id: string) => {
    console.log(id);

    try {
      const resp = await invoke("delete_library", { id: id.split(":") });
      reload();
    } catch (e) {
      alert(e);
    }
  };

  const scan = async (id: string) => {
    try {
      const resp = await invoke("scan_library", {
        id: id.split(":"),
        extension: "stl",
      });
    } catch (e) {
      alert(e);
    }
  };

  onMount(async () => {
    reload();
  });
</script>

<div>
  <h2 class="text-3xl">Add Library</h2>
  <h3 class="text-xl">Existing Libraries</h3>
  <ul class="list-disc">
    {#each libraries as library (library.id)}
      <li class="list-item">
        {library.name} - {library.path}
        <button
          class="btn"
          on:click={() => {
            console.log(library);
            deleteLib(library.id);
          }}>Delete</button
        >
        <button class="btn" on:click={() => scan(library.id)}>Scan</button>
      </li>
    {/each}
  </ul>
  <div class="form-control py-2">
    <input class="input input-bordered" type="text" bind:value={name} />
  </div>
  <div class="form-control py-2">
    Library: {library}
  </div>
  <div class="form-control py-2">
    <button class="btn" on:click={() => load()}>Dir</button>
  </div>
  <div class="form-control py-2">
    <button class="btn" on:click={() => click()}>Save</button>
  </div>
</div>
