<script lang="ts">
  import FileViewer from "./components/FileViewer.svelte";
  import type { STLFile } from "./lib/file";
  import "./styles.css";
  let testData: STLFile[] = [
    {
      name: "test-file1.stl",
      path: "/Users/test/stls/test-file1.stl",
    },
    {
      name: "test-file2.stl",
      path: "/Users/test/stls/test-file2.stl",
    },
    {
      name: "test-file3.stl",
      path: "/Users/test/stls/test-file3.stl",
    },
  ];
</script>

<main class="container p-5">
  <h1 class="text-3xl">Title</h1>
  <FileViewer files={testData} />
</main>
