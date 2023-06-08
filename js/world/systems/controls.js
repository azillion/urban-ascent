import { MapControls } from 'three/examples/jsm/controls/MapControls';
import { world } from '..';

function createControls(camera, canvas) {
	const controls = new MapControls(camera, canvas);
	controls.enableDamping = true;
	controls.dampingFactor = 0.1;
	controls.minDistance = 5;
	controls.maxDistance = 200;
	// controls.minAzimuthAngle = -Infinity; // default
	// controls.maxAzimuthAngle = Infinity; // default
	// controls.minPolarAngle = 0; // default
	controls.maxPolarAngle = Math.PI / 2 - 0.1;
	controls.keys = {
		LEFT: 'KeyA', // A
		UP: 'KeyW', // W
		RIGHT: 'KeyD', // D
		BOTTOM: 'KeyS' // S
	};
	controls.keyPanSpeed = 15;

	controls.tick = () => {
		if (world.isDragging) return;
		// if (controls.enabled === false) return;

		if (controls.target.y < 0) {
			controls.target.y = 0;
		}
		controls.update();
	}

	return controls;
}

export { createControls };