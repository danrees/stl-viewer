<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  type Tag = {
    id: string;
    value: string;
  };
  let tags: Promise<Tag[]> = invoke("get_tags");
</script>

<div>
  <h1>Tags</h1>
  {#await tags}
    <div>Loading ...</div>
  {:then tag}
    <ul>
      {#each tag as t}
        <li>{t.value}</li>
      {/each}
    </ul>
  {:catch error}
    <div>{error}</div>
  {/await}
  <ul />
</div>
