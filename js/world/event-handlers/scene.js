import * as THREE from 'three';

import { world } from '..';
import { globalEventManager } from '../../services/GlobalEventManager';
import { WORLD_EVENTS } from '../world';

function initializeSceneEventHandlers() {
	globalEventManager.registerEventListener(WORLD_EVENTS.TOGGLE_DAY_NIGHT, handleToggleDayNight);
}

function handleToggleDayNight(isDay) {
	if (!world) return;

	const scene = world.scene();
	if (!scene) return;

	const directionalLight = scene.getObjectByName('main-light');
	if (!directionalLight) return;
	directionalLight.intensity = isDay ? 8 : 0.5;

	const ambientLight = scene.getObjectByName('ambient-light');
	if (!ambientLight) return;
	ambientLight.intensity = isDay ? 3 : 5;

	scene.background = isDay ? new THREE.Color('skyblue') : new THREE.Color(0x191970);
}

export default initializeSceneEventHandlers;