import { PlaneGeometry, MathUtils, Mesh, MeshStandardMaterial } from 'three';

function createMaterials() {
  // const textureLoader = new TextureLoader();

  // const texture = textureLoader.load(
  //   '/assets/textures/uv-test-bw.png',
  // );
  // create a "standard" material
  const material = new MeshStandardMaterial({
    color: "#819fb3",
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

function createRoad(length = 2) {
  // create a geometry
  const geometry = new PlaneGeometry(3.5, length, 1, 1);

  // create a default (white) Basic material
  const material = createMaterials();

  // create a Mesh containing the geometry and material
  const road = new Mesh(geometry, material);
  
  road.rotateX(MathUtils.degToRad(-90));
  road.rotateZ(MathUtils.degToRad(90));

  return road;
}

export { createRoad };