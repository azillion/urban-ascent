import { globalEventManager } from '../../services/GlobalEventManager';
import { DragManager } from '../systems/DragManager';

const EVENTS = Object.freeze({
	BUILDING_MOUSE_DOWN: 'buildingMouseDown',
	BUILDING_MOUSE_MOVE: 'buildingMouseMove',
	BUILDING_MOUSE_UP: 'buildingMouseUp',
	BUILDING_CLICK: 'buildingClick',
});

class BuildingsManager {
	constructor() {
		this.buildingHashMap = new Map();
	}

	addBuilding(building) {
		this.buildingHashMap.set(building.mesh.id, building);
	}

	removeBuilding(building) {
		this.buildingHashMap.delete(building.mesh.id);
	}

	getBuilding(id) {
		return this.buildingHashMap.get(id);
	}
}

const buildingsManager = new BuildingsManager();

class Building {
	constructor(mesh) {
		this.mesh = mesh;
		this.dragManager = new DragManager(this.mesh);
		this.registerBrowserEvents();
	}

	registerBrowserEvents() {
		const handleEvent = (gameEvent) => (intersections) => {
			if (this.mesh && intersections.length > 0 && intersections[0].object.id === this.mesh.id) {
				globalEventManager.dispatchEvent(gameEvent, intersections);
			}
		};
		globalEventManager.registerEventListener('mousedown', handleEvent(EVENTS.BUILDING_MOUSE_DOWN));
		globalEventManager.registerEventListener('mouseup', handleEvent(EVENTS.BUILDING_MOUSE_UP));
		globalEventManager.registerEventListener('mousemove', handleEvent(EVENTS.BUILDING_MOUSE_MOVE));
		globalEventManager.registerEventListener('click', handleEvent(EVENTS.BUILDING_CLICK));
	}
}

export { Building, buildingsManager, EVENTS as BUILDING_EVENTS };