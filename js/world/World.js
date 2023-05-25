import { Raycaster, Vector2 } from 'three';

import { createScene } from './components/scene';
import { createCamera } from './components/camera';
import { createCube } from './components/cube';
import { createLights } from './components/lights';
import { createTerrain } from './components/terrain';
// import { createRoad } from './components/road';
// import { createBunchOfShit } from './components/performance';
// import { PathGroup } from './components/paths';
// import { createClouds } from './components/clouds';

import { createRenderer } from './systems/renderer';
import { Resizer } from './systems/Resizer';
import { Loop } from './systems/Loop';
import { createControls } from './systems/controls';

import { GameManager } from '../../pkg';
import { globalState } from '../services/GlobalState';
import { globalEventManager } from '../services/GlobalEventManager';
import { initializeWorldEventHandlers } from './event-handlers';

const WORLD_EVENTS = Object.freeze({
	TOGGLE_DAY_NIGHT: 'toggleDayNight',
});

let camera;
let renderer;
let scene;
let loop;
let resizer;
let mouse;
let raycaster;

class World {
	constructor(container) {
		this.gameManager = GameManager.new();
		this.globalState = globalState;
		camera = createCamera();
		renderer = createRenderer();
		scene = createScene();
		loop = new Loop(camera, scene, renderer);
		
		container.appendChild(renderer.domElement);
		this.canvas = renderer.domElement;
		
		const controls = createControls(camera, this.canvas);
		// const skybox = createSkybox();
		// const clouds = createClouds(100, 100, 1.5);

		const cube = createCube();
		const terrain = createTerrain();
		terrain.add(cube);
		cube.position.y = cube.geometry.parameters.height / 2 + 0.1;
		// const pathGroup = new PathGroup([new Vector2(5, 0), new Vector2(15, 10)], false);
		// const path = pathGroup.getPath();
		// terrain.add(path);
		// path.position.set(5, 0.1, 0);

		// const road = createRoad(15);
		// terrain.add(road);
		// road.position.set(5, 0.1, 0);

		// const bunchOfShit = createBunchOfShit();

		// terrain.add(bunchOfShit);

		const { mainLight, ambientLight } = createLights();
		
		scene.add(terrain, mainLight, ambientLight);

		controls.target.copy(terrain.position);
		controls.enabled = false;
		camera.position.set(-40,30,40);
		camera.lookAt(cube.position);

		// loop.updatables.push(cube);
		loop.updatables.push(this.gameManager)
		loop.updatables.push(controls);
		// loop.updatables.push(bunchOfShit);

		resizer = new Resizer(container, camera, renderer);
		resizer.registerEvents();

		mouse = new Vector2();
		raycaster = new Raycaster();
		this.setupInteractions();

		// controls.addEventListener('change', () => {
		// 	// this.render();
		// });
		
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

	scene() {
		return scene;
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

	setupInteractions() {
		const canvas = renderer.domElement;
		const mouseEventHandler = (eventString) => {
				canvas.addEventListener(eventString, (event) => {
					// Calculate normalized device coordinates (-1 to +1) for the mouse
					mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
					mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

					// Set the raycaster's origin and direction based on the mouse position
					raycaster.setFromCamera(mouse, camera);

					var intersects = raycaster.intersectObjects(scene.children, true);
					globalEventManager.dispatchEvent(eventString, intersects);
			}, false);
		};
		mouseEventHandler('click');
		mouseEventHandler('mousemove');
		mouseEventHandler('mousedown');
		mouseEventHandler('mouseup');
		// mouseEventHandler('touchstart');
		// mouseEventHandler('touchend');
		// mouseEventHandler('touchmove');
		// mouseEventHandler('touchcancel');
		// mouseEventHandler('pointerdown');
		// mouseEventHandler('pointerup');
		// mouseEventHandler('pointermove');
		// mouseEventHandler('pointerover');
		// mouseEventHandler('pointerout');
		// mouseEventHandler('pointerenter');
		// mouseEventHandler('pointerleave');
		// mouseEventHandler('wheel');
		// mouseEventHandler('contextmenu');
		// mouseEventHandler('dblclick');
		// mouseEventHandler('drag');
		// mouseEventHandler('dragend');
		// mouseEventHandler('dragenter');
		// mouseEventHandler('dragexit');
		// mouseEventHandler('dragleave');
		// mouseEventHandler('dragover');
		// mouseEventHandler('dragstart');
		initializeWorldEventHandlers();
	}
}

export { World, WORLD_EVENTS };