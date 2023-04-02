<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import FileViewer from "./components/FileViewer.svelte";
  import type { STLFile } from "./lib/file";
  import "./styles.css";
  import Router from "svelte-spa-router";
  import { routes } from "./routes";
</script>

<main class="container p-5">
  <div class="tabs">
    <a href="#/" class="tab tab-bordered">Home</a>
    <a href="#/library" class="tab tab-bordered">Libraries</a>
    <a href="#/files" class="tab tab-bordered">Files</a>
  </div>
  <Router {routes} />
</main>
