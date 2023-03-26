<script lang="ts">
  import type { STLFile } from "src/lib/file";

  let filter: string;
  export let files: STLFile[];
</script>

<div>
  <input class="input" type="text" placeholder="Filter" bind:value={filter} />
  <ul class="list-disc">
    {#each files as file (file.path)}
      <li class="list-item">{file.name}</li>
    {/each}
  </ul>
</div>
