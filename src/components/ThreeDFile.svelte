<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import type { STLFile } from "src/lib/file";
  import File from "../components/File.svelte";

  let promise: Promise<STLFile[]> = invoke("list_files");
  let tagFilters: string[] = [];

  const addFilters = (filter: string) => {
    const i = tagFilters.findIndex((f) => f == filter);
    if (i < 0) {
      tagFilters.push(filter);
      tagFilters = [...tagFilters];
    } else {
      tagFilters = [...tagFilters.slice(0, i), ...tagFilters.slice(i + 1)];
    }
  };
</script>

{#await promise}
  <div>Loading...</div>
{:then files}
  <p>Filters</p>
  {#each tagFilters as filter}
    <span class="badge" on:click={() => addFilters(filter)}>{filter}</span>
  {/each}
  <ul>
    {#each files as f (f.id)}
      <li>
        <File stl={f} filter={addFilters} />
      </li>
    {/each}
  </ul>
{/await}
