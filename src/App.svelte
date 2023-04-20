<script lang="ts">
  import "./styles.css";
  import Router from "svelte-spa-router";
  import { routes } from "./routes";
  import { location } from "svelte-spa-router";
</script>

<main class="container mx-auto p-5">
  <div class="tabs">
    <a href="#/" class="tab tab-bordered" class:tab-active={$location == "/"}
      >Home</a
    >
    <a
      href="#/library"
      class="tab tab-bordered"
      class:tab-active={$location.includes("library")}>Libraries</a
    >
    <a
      href="#/files"
      class="tab tab-bordered"
      class:tab-active={$location.includes("files")}>Files</a
    >
    <a
      href="#/tags"
      class="tab tab-bordered"
      class:tab-active={$location.includes("tags")}>Tags</a
    >
  </div>
  <Router {routes} />
</main>
