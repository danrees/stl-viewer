import Home from "./components/Home.svelte";
import Library from "./components/Library.svelte";
import Tag from "./components/Tag.svelte";
import ThreeDFile from "./components/ThreeDFile.svelte";
import Viewer from "./components/Viewer.svelte";

const routes = {
  "/": Home,
  "/library": Library,
  "/files": ThreeDFile,
  "/tags": Tag,
  "/viewer/:id": Viewer,
};

export { routes };
