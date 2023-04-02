<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";

  type File = { name: string; path: string };
  let files: File[] = [];

  const loadFiles = async () => {
    const resp: File[] = await invoke("list_files");
    files = resp;
  };

  onMount(() => {
    loadFiles();
  });
</script>

<ul>
  {#each files as f}
    <li>{f.name} - {f.path}</li>
  {/each}
</ul>
