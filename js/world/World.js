import { createScene } from './components/scene';
import { createCamera } from './components/camera';
import { createCube } from './components/cube';
import { createLights } from './components/lights';

import { createRenderer } from './systems/renderer';
import { Resizer } from './systems/Resizer';
import { Loop } from './systems/Loop';
import { createControls } from './systems/controls';
import { createTerrain } from './components/terrain';
import { createRoad } from './components/road';

let camera;
let renderer;
let scene;
let loop;
let resizer;

class World {
	constructor(container) {
		camera = createCamera();
		renderer = createRenderer();
		scene = createScene();
		loop = new Loop(camera, scene, renderer);
		
		container.appendChild(renderer.domElement);
		this.canvas = renderer.domElement;
		
		const controls = createControls(camera, this.canvas);
		const cube = createCube();
		const terrain = createTerrain();
		terrain.add(cube);
		cube.position.y = cube.geometry.parameters.height / 2 + 0.1;
		const road = createRoad(15);
		terrain.add(road);
		road.position.set(5, 0.1, 0);
		const { mainLight, ambientLight} = createLights();
		
		scene.add(terrain, cube, mainLight, ambientLight);

		controls.target.copy(terrain.position);
		camera.position.set(0,40,10);
		// camera.position.y = 20;
		// terrain.com
		camera.lookAt(terrain.position);

		// loop.updatables.push(cube);
		loop.updatables.push(controls);

		resizer = new Resizer(container, camera, renderer);

		controls.addEventListener('change', () => {
			this.render();
		});
	}

	async init() {
		await this.loadAssets();
	}

	async loadAssets() {
		// todo load assets
		// await loadTrees();
		// await loadRocks();
		// await loadPeople();
		// await loadVehicles();
		// await loadBuildings();
		// await loadGround();
		// await loadSky();
		// await loadWater();
		// await loadClouds();
		// await loadSun();
		// await loadMoon();
	}

	render() {
		renderer.render(scene, camera);
	}

	start() {
		loop.start();
	}

	stop() {
		loop.stop();
	}

	addToLoop(object) {
		loop.updatables.push(object);
	}

	removeFromLoop(id) {
		const index = loop.updatables.findIndex(object => object.id === id);

		if (index !== -1) {
			loop.updatables.splice(index, 1);
		}
	}
}

export { World };