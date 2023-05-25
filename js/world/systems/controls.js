import { MapControls } from 'three/examples/jsm/controls/MapControls';

function createControls(camera, canvas) {
	const controls = new MapControls(camera, canvas);
	controls.enableDamping = true;
	controls.minDistance = 5;
	controls.maxDistance = 300;
	// controls.minAzimuthAngle = -Infinity; // default
	// controls.maxAzimuthAngle = Infinity; // default
	// controls.minPolarAngle = 0; // default
	controls.maxPolarAngle = Math.PI / 2 - 0.1;
	// controls.keys = {
	// 	LEFT: 65, // A
	// 	UP: 87, // W
	// 	RIGHT: 68, // D
	// 	BOTTOM: 83 // S
	// };

	controls.tick = () => {
		if (controls.enabled === false) return;

		if (controls.target.y < 0) {
			controls.target.y = 0;
		}
		controls.update();
	}

	return controls;
}

export { createControls };