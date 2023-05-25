import { DirectionalLight, HemisphereLight } from 'three';

function createLights() {
	const mainLight = new DirectionalLight('white', 8);
	mainLight.name = 'main-light';
	const ambientLight = new HemisphereLight(
		'white', // bright sky color
		'darkslategrey', // dim ground color
		3, // intensity
	);
	ambientLight.name = 'ambient-light';

	mainLight.position.set(10, 10, 10);

	return { mainLight, ambientLight };
}

export { createLights };