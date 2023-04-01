<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { STLFile } from "src/lib/file";
  import FileViewer from "./FileViewer.svelte";

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

  const scan = async () => {
    await invoke("scan_libraries");
  };
</script>

<h1 class="text-3xl">Title</h1>
<FileViewer files={testData} />
<button
  class="btn"
  on:click={async () => {
    await scan();
  }}>Scan</button
>
