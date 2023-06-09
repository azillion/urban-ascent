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

import { globalState } from '../services/GlobalState';
import { globalEventManager } from '../services/GlobalEventManager';
import { initializeWorldEventHandlers } from './event-handlers';
import { createGrid } from './components/grid';
import { UrbanAscent } from '../../pkg';

const WORLD_EVENTS = Object.freeze({
	TOGGLE_DAY_NIGHT: 'toggleDayNight',
	TOGGLE_PAUSE_PLAY: 'togglePausePlay',
	GAME_HAS_LOADED: 'gameHasLoaded',
	TOOL_MODE_CHANGED: 'toolModeChanged',
});

const DEFAULT_WIDTH = 1000;
const DEFAULT_HEIGHT = 1000;

let camera;
let renderer;
let scene;
let loop;
let resizer;
let mouse;
let raycaster;
let terrain;
let controls

class World {
	constructor(container) {
		this.gameManager = new UrbanAscent();
		globalState.gameManager = this.gameManager;
		this.globalState = globalState;
		this.loadGame();
		this.gridSize = {
			width: this.gameManager.getGridWidth() || DEFAULT_WIDTH,
			height: this.gameManager.getGridHeight() || DEFAULT_HEIGHT,
		};
		camera = createCamera();
		renderer = createRenderer();
		scene = createScene();
		loop = new Loop(camera, scene, renderer);
		
		container.appendChild(renderer.domElement);
		this.canvas = renderer.domElement;
		
		controls = createControls(camera, this.canvas);

		terrain = createTerrain(this.gridSize.width, this.gridSize.height);

		// origin cube
		const cube = createCube();
		terrain.add(cube);
		cube.position.y = cube.geometry.parameters.height / 2 + 0.1;

		const grid = createGrid(this.gridSize.width, this.gridSize.height);
		grid.visible = false;
		scene.add(grid);

		const { mainLight, ambientLight } = createLights();
		
		scene.add(terrain, mainLight, ambientLight);

		controls.target.copy(terrain.position);
		camera.position.set(-40,50,60);
		camera.lookAt(cube.position);

		loop.updatables.push(this.gameManager) // runs the game loop
		loop.updatables.push(controls);

		resizer = new Resizer(container, camera, renderer);
		resizer.registerEvents();

		mouse = new Vector2();
		raycaster = new Raycaster();
		this.setupInteractions();

		this.isDragging = false;
		this.draggedEntity = null;

		this.controlsOffAndRotate();
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

	loadGame() {
		const isLoaded = this.gameManager.loadGame();
		if (!isLoaded) {
			this.gameManager.newGame('Tutoria', 'azillion');
			this.gameManager.loadGame();
		}
		console.log(this.gameManager.getTownName());
	}

	get scene() {
		return scene;
	}

	get camera() {
		return camera;
	}

	get domElement() {
		return renderer.domElement;
	}

	get terrain() {
		return terrain;
	}

	get controls() {
		return controls;
	}

	controlsOffAndRotate() {
		controls.enabled = false;
		controls.autoRotate = true;
		try {
			controls.stopListenToKeyEvents();
		} catch(e) { /* empty */ }
		controls.update();
	}

	controlsOnAndStopRotate() {
		controls.enabled = true;
		controls.autoRotate = false;
		controls.listenToKeyEvents(window);
		controls.update();
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
					mouse.x = (event.clientX / renderer.domElement.clientWidth) * 2 - 1;
					mouse.y = -(event.clientY / renderer.domElement.clientHeight) * 2 + 1;

					// Set the raycaster's origin and direction based on the mouse position
					raycaster.setFromCamera(mouse, camera);

					const intersects = raycaster.intersectObjects(scene.children, true);
					if (intersects.length > 0)
						globalEventManager.dispatchEvent(eventString, intersects, event);
			}, false);
		};
		mouseEventHandler('click');
		mouseEventHandler('mousemove');
		mouseEventHandler('mousedown');
		mouseEventHandler('mouseup');
		canvas.addEventListener('contextmenu', (event) => event.preventDefault());
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