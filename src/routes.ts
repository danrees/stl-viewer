import Home from "./components/Home.svelte";
import Library from "./components/Library.svelte";
import ThreeDFile from "./components/ThreeDFile.svelte";

const routes = {
  "/": Home,
  "/library": Library,
  "/files": ThreeDFile,
};

export { routes };
