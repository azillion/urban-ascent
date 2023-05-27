import * as THREE from 'three';

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

	checkCollision(box1, box2) {
		// Create bounding boxes for the two boxes
		const boundingBox1 = new THREE.Box3().setFromObject(box1);
		const boundingBox2 = new THREE.Box3().setFromObject(box2);

		// Check for collision
		const hasCollided = boundingBox1.intersectsBox(boundingBox2);

		return hasCollided;
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
		const handleEvent = (gameEvent) => (intersections, event) => {
			if (this.mesh) {
				globalEventManager.dispatchEvent(gameEvent, intersections, event);
			}
		};
		globalEventManager.registerEventListener('mousedown', handleEvent(EVENTS.BUILDING_MOUSE_DOWN));
		globalEventManager.registerEventListener('mouseup', handleEvent(EVENTS.BUILDING_MOUSE_UP));
		globalEventManager.registerEventListener('mousemove', handleEvent(EVENTS.BUILDING_MOUSE_MOVE));
		globalEventManager.registerEventListener('click', handleEvent(EVENTS.BUILDING_CLICK));
	}
}

export { Building, buildingsManager, EVENTS as BUILDING_EVENTS };