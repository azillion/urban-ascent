import { Color, Fog, Scene } from 'three';

const SKY_COLOR_DAY = 'skyblue';
const SKY_COLOR_NIGHT = 'midnightblue';

function createScene() {
	const scene = new Scene();
	const color = new Color(SKY_COLOR_DAY);

	scene.background = color;
	const near = 100;
	const far = 400;
	scene.fog = new Fog(color, near, far);

	return scene;
}

export { createScene, SKY_COLOR_DAY, SKY_COLOR_NIGHT };