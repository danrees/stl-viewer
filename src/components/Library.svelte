<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from "@tauri-apps/api/dialog";
  import { onMount } from "svelte";
  import type { Library } from "../lib/file";
  import { path } from "@tauri-apps/api";

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
    if (name === undefined) {
      const parts = resp.split(path.sep);
      name = parts.at(-1);
    }
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

<div class="">
  <h2 class="text-3xl">Add Library</h2>
  <h3 class="text-xl">Existing Libraries</h3>
  <ul class="px-8 py-4">
    {#each libraries as library (library.id)}
      <li class="list-item py-2">
        <div class="card card-compact bg-base-100 shadow-xl">
          <div class="card-body">
            <h4 class="card-title">{library.name}</h4>
            <p>{library.path}</p>
            <div class="card-actions">
              <button
                class="btn"
                on:click={() => {
                  console.log(library);
                  deleteLib(library.id);
                }}>Delete</button
              >
              <button class="btn" on:click={() => scan(library.id)}>Scan</button
              >
            </div>
          </div>
        </div>
      </li>
    {/each}
  </ul>
  <div class="form-control py-2">
    <div class="input-group">
      <button class="btn" on:click={() => load()}>Dir</button>
      <input class="input input-bordered" type="text" bind:value={name} />
    </div>
    <div class="py-2">
      <button class="btn" on:click={() => click()}>Save</button>
    </div>
  </div>
  {#if library}
    <div class="form-control py-2">
      Library: {library}
    </div>
  {/if}
  <div class="form-control py-2" />
</div>
