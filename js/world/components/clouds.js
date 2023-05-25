import * as THREE from 'three';

function createClouds(radius, segments, cloudiness) {
  // Create a sphere geometry
  let geometry = new THREE.SphereGeometry(radius, segments, segments);

  // Create a vertex displacement array
  let positionAttribute = geometry.getAttribute('position');

  // Get the array buffer that contains the vertices
  let vertices = positionAttribute.array;

  // Get the number of vertices
  let vertexCount = positionAttribute.count;

  // Loop through the vertices
  for (let i = 0; i < vertexCount; i++) {
    // Retrieve the x, y, z components of each vertex
    let x = vertices[i * 3];
    let y = vertices[i * 3 + 1];
    let z = vertices[i * 3 + 2];
    const v = new THREE.Vector3(x, y, z);

    // Displace the vertices randomly based on cloudiness
    let displacement = new THREE.Vector3();
    displacement.x = (Math.random() - 0.5) * cloudiness * 10;
    displacement.y = (Math.random() - 0.5) * cloudiness * 10;
    displacement.z = (Math.random() - 0.5) * cloudiness * 10;
    
    // set vertex to sum of original position and displacement
    v.add(displacement);
    v.round();
    vertices[i * 3] = v.x;
    vertices[i * 3 + 1] = v.y;
    vertices[i * 3 + 2] = v.z;
  }

  // Create a material for the clouds
  let material = new THREE.MeshLambertMaterial({
    color: 0xffffff,
    opacity: 0.8,
    transparent: true,
    blending: THREE.AdditiveBlending
  });

  // Create a mesh using the geometry and material
  let clouds = new THREE.Mesh(geometry, material);

  clouds.position.y = 100;
  clouds.rotateZ(THREE.MathUtils.degToRad(-90));

  // Return the clouds mesh
  return clouds;
}

export { createClouds };
