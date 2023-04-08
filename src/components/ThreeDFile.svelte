<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import type { STLFile } from "src/lib/file";
  import File from "../components/File.svelte";

  let promise: Promise<STLFile[]> = invoke("list_files");
</script>

{#await promise}
  <div>Loading...</div>
{:then files}
  <ul>
    {#each files as f (f.id)}
      <li>
        <File stl={f} />
      </li>
    {/each}
  </ul>
{/await}
