import { BoxBufferGeometry, MathUtils, Mesh, MeshStandardMaterial } from 'three';

function createCube() {
	// create a geometry
	const geometry = new BoxBufferGeometry(5, 3, 5);

	// create a default (white) Basic material
	const material = new MeshStandardMaterial({
		color: "firebrick",
	});

	// create a Mesh containing the geometry and material
	const cube = new Mesh(geometry, material);
	cube.castShadow = true;

	const radiansPerSecond = MathUtils.degToRad(30);

	// this method will be called once per frame
	cube.tick = (delta) => {
		// increase the cube's rotation each frame
		const random = Math.random() * 10;
		delta = delta * random;
		cube.rotation.z += radiansPerSecond * delta;
		cube.rotation.x += radiansPerSecond * delta;
		cube.rotation.y += radiansPerSecond * delta;
		// bounce position back and forth
		if (cube.position.x < -3) {
			cube.position.x = 3;
		} else {
			cube.position.x += -0.1;
		}
		if (cube.position.y < -3) {
			cube.position.y = 3;
		} else {
			cube.position.y += -0.1;
		}
	};

	return cube;
}

export { createCube };