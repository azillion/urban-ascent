import { BoxBufferGeometry, MathUtils, Mesh, MeshStandardMaterial } from 'three';

function createMaterials() {
  // const textureLoader = new TextureLoader();

  // const texture = textureLoader.load(
  //   '/assets/textures/uv-test-bw.png',
  // );
  // create a "standard" material
  const material = new MeshStandardMaterial({
    color: "firebrick",
    // flatShading: true,
  });

  // // create a "standard" material
  // const detail = new MeshStandardMaterial({
  //   color: "darkslategray",
  //   flatShading: true,
  // });

  // return { body, detail };
  return material;
}

function createCube() {
  // create a geometry
  const geometry = new BoxBufferGeometry(5, 3, 5);

  // create a default (white) Basic material
  const material = createMaterials();

  // create a Mesh containing the geometry and material
  const cube = new Mesh(geometry, material);
  cube.castShadow = true;

  const radiansPerSecond = MathUtils.degToRad(30);

  // this method will be called once per frame
  cube.tick = (delta) => {
    // increase the cube's rotation each frame
    cube.rotation.z += radiansPerSecond * delta;
    cube.rotation.x += radiansPerSecond * delta;
    cube.rotation.y += radiansPerSecond * delta;
  };

  return cube;
}

export { createCube };