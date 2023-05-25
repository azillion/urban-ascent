// import throttle from "lodash/throttle";

const setSize = (container, camera, renderer) => {
  camera.aspect = container.clientWidth / container.clientHeight;
  camera.updateProjectionMatrix();

  renderer.setSize(container.clientWidth, container.clientHeight);
  renderer.setPixelRatio(window.devicePixelRatio);
};

class Resizer {
  constructor(container, camera, renderer) {
    setSize(container, camera, renderer);
  }

  registerEvents() {
    window.addEventListener("resize", () => {
      // set the size again if a resize occurs
      // throttle(() => setSize(container, camera, renderer), 200);
      setSize(container, camera, renderer);
    });
  }
}

export { Resizer };