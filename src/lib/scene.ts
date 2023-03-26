import * as three from "three";
import { FileLoader } from "three";
import { STLLoader } from "three/examples/jsm/loaders/STLLoader";

const createScene = (
  el: HTMLCanvasElement,
  window: Window,
  path: ArrayBuffer
) => {
  const camera = new three.PerspectiveCamera(
    75,
    window.innerWidth / window.innerHeight,
    0.1,
    1000
  );
  // const geometry = new three.BoxGeometry();
  // const material = new three.MeshBasicMaterial({ color: 0x00ff00 });
  // const cube = new three.Mesh(geometry, material);

  camera.position.z = 5;

  const renderer = new three.WebGLRenderer({ antialias: true, canvas: el });
  renderer.setSize(window.innerWidth, window.innerHeight);

  const scene = new three.Scene();
  scene.add(new three.HemisphereLight(0xffffff, 1.5));

  const loader = new STLLoader();
  const fLoader = new FileLoader();

  const geometry = loader.parse(path);

  const material = new three.MeshPhongMaterial({
    color: 0xff5533,
    specular: 100,
    shininess: 100,
  });
  const mesh = new three.Mesh(geometry, material);
  scene.add(mesh);
  var middle = new three.Vector3();
  geometry.computeBoundingBox();
  geometry.boundingBox.getCenter(middle);
  mesh.geometry.applyMatrix4(
    new three.Matrix4().makeTranslation(-middle.x, -middle.y, -middle.z)
  );
  var largestDimension = Math.max(
    geometry.boundingBox.max.x,
    geometry.boundingBox.max.y,
    geometry.boundingBox.max.z
  );

  camera.position.z = largestDimension * 1.5;

  renderer.render(scene, camera);
  return renderer;
};

export { createScene };
