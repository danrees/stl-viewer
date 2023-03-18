import * as three from "three";

import { STLLoader } from "three/examples/jsm/loaders/STLLoader";

function createScene(
  canvas: HTMLCanvasElement,
  size: { height: number; width: number },
  url: string
) {
  const renderer = new three.WebGLRenderer({ antialias: true, canvas: canvas });
  const camera = new three.PerspectiveCamera(
    75,
    size.width / size.height,
    0.1,
    1000
  );
  renderer.setSize(size.width, size.height);
  camera.aspect = size.width / size.height;
  camera.updateProjectionMatrix();
  const scene = new three.Scene();
  scene.add(new three.HemisphereLight(0xffffff, 1.5));

  const loader = new STLLoader();
  loader.load(url, (geometry) => {
    const material = new three.MeshPhongMaterial({
      color: 0xff5533,
      specular: 100,
      shininess: 100,
    });
    const mesh = new three.Mesh(geometry, material);
    scene.add(mesh);

    const middle = new three.Vector3();
    geometry.computeBoundingBox();
    geometry.boundingBox.getCenter(middle);
    mesh.geometry.applyMatrix4(
      new three.Matrix4().makeTranslation(-middle.x, -middle.y, -middle.z)
    );
    const largestDim = Math.max(
      geometry.boundingBox.max.x,
      geometry.boundingBox.max.y,
      geometry.boundingBox.max.z
    );
    camera.position.z = largestDim * 1.5;
  });
}
