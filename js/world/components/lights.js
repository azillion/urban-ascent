import { DirectionalLight, HemisphereLight } from 'three';

const MAIN_LIGHT_INTENSITY = 8;
const MAIN_LIGHT_NAME = 'main-light';
const AMBIENT_LIGHT_INTENSITY = 3;
const AMBIENT_LIGHT_NAME = 'ambient-light';

function createLights() {
	const mainLight = new DirectionalLight('white', MAIN_LIGHT_INTENSITY);
	mainLight.name = MAIN_LIGHT_NAME;
	const ambientLight = new HemisphereLight(
		'white', // bright sky color
		'darkslategrey', // dim ground color
		AMBIENT_LIGHT_INTENSITY, // intensity
	);
	ambientLight.name = AMBIENT_LIGHT_NAME;
	mainLight.castShadow = true;

	mainLight.position.set(100, 100, 10);

	return { mainLight, ambientLight };
}



export { createLights, MAIN_LIGHT_NAME, AMBIENT_LIGHT_NAME, MAIN_LIGHT_INTENSITY, AMBIENT_LIGHT_INTENSITY };