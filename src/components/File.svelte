<script lang="ts">
  import type { STLFile } from "src/lib/file";

  export let stl: STLFile;
  export let load: () => string;
</script>

<div class="card">
  <div class="card-body">
    <h2 class="card-title">{stl.name}</h2>
    <p>{stl.path}</p>
    <button
      class="btn"
      on:click={() => {
        load();
      }}>Load</button
    >
  </div>
</div>
