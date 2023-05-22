import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';

function createControls(camera, canvas) {
	const controls = new OrbitControls(camera, canvas);
	controls.enableDamping = true;
	controls.minDistance = 5;
	controls.maxDistance = 300;
	// controls.minAzimuthAngle = -Infinity; // default
	// controls.maxAzimuthAngle = Infinity; // default
	// controls.minPolarAngle = 0; // default
	controls.maxPolarAngle = Math.PI / 2 - 0.1; // default

	controls.tick = () => {
		if (controls.enabled === false) return;
		// dont allow the controls to go below zero

		if (camera.position.y < 0) {
			camera.position.y = 0;
		}
		controls.update();
	}

	return controls;
}

export { createControls };