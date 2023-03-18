import * as three from "three";

const createScene = (el: HTMLCanvasElement, window: Window) => {
  const camera = new three.PerspectiveCamera(
    75,
    window.innerWidth / window.innerHeight,
    0.1,
    1000
  );
  const geometry = new three.BoxGeometry();
  const material = new three.MeshBasicMaterial({ color: 0x00ff00 });
  const cube = new three.Mesh(geometry, material);

  const scene = new three.Scene();
  scene.add(cube);
  camera.position.z = 5;

  const renderer = new three.WebGLRenderer({ antialias: true, canvas: el });
  renderer.setSize(window.innerWidth, window.innerHeight);
  camera.aspect = window.innerWidth / window.innerHeight;
  camera.updateProjectionMatrix();

  function animate() {
    window.requestAnimationFrame(animate);
    cube.rotation.x += 0.01;
    cube.rotation.y += 0.01;
    renderer.render(scene, camera);
  }

  window.requestAnimationFrame(animate);
  return renderer;
};

export { createScene };
