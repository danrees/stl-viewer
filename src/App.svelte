<script lang="ts">
  import "./styles.css";
  import Router from "svelte-spa-router";
  import { routes } from "./routes";
</script>

<main class="container p-5">
  <div class="tabs">
    <a href="#/" class="tab tab-bordered">Home</a>
    <a href="#/library" class="tab tab-bordered">Libraries</a>
    <a href="#/files" class="tab tab-bordered">Files</a>
    <a href="#/tags" class="tab tab-bordered">Tags</a>
  </div>
  <Router {routes} />
</main>
