<script lang="ts">
  import type { STLFile } from "src/lib/file";

  export let stl: STLFile;
  export let filter: (string) => void;
</script>

<div class="card">
  <div class="card-body">
    <h2 class="card-title">{stl.name}</h2>
    <p>{stl.path}</p>
    <h3 class="text-xl">Tags</h3>

    <div class="card-actions">
      {#each stl.tags as tag}
        <span class="badge badge-accent" on:click={() => filter(tag.value)}
          >{tag.value}</span
        >
      {/each}
    </div>

    <a href="#/viewer/{stl.id}" class="link">View</a>
  </div>
</div>
