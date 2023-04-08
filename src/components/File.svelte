<script lang="ts">
  import type { STLFile } from "src/lib/file";

  export let stl: STLFile;
</script>

<div class="card">
  <div class="card-body">
    <h2 class="card-title">{stl.name}</h2>
    <p>{stl.path}</p>
    <h3 class="text-xl">Tags</h3>
    <ul class="px-10">
      {#each stl.tags as tag}
        <li><span class="badge badge-accent">{tag.value}</span></li>
      {/each}
    </ul>
  </div>
</div>
