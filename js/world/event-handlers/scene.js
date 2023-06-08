import * as THREE from 'three';

import { world } from '..';
import { globalEventManager } from '../../services/GlobalEventManager';
import { WORLD_EVENTS } from '../world';
import { AMBIENT_LIGHT_INTENSITY, AMBIENT_LIGHT_NAME, MAIN_LIGHT_INTENSITY, MAIN_LIGHT_NAME } from '../components/lights';
import { SKY_COLOR_DAY, SKY_COLOR_NIGHT } from '../components/scene';

function initializeSceneEventHandlers() {
	globalEventManager.registerEventListener(WORLD_EVENTS.TOGGLE_DAY_NIGHT, handleToggleDayNight);
	globalEventManager.registerEventListener(WORLD_EVENTS.GAME_HAS_LOADED, handleGameHasLoaded);
}

function handleGameHasLoaded(hasGameLoaded) {
	if (!world) return;
	if (!world.scene) return;

	if (hasGameLoaded) {
		world.controlsOnAndStopRotate();
	} else {
		world.controlsOffAndStartRotate();
	}
}

function handleToggleDayNight(isDay) {
	if (!world) return;

	const scene = world.scene;
	if (!scene) return;

	const mainLight = scene.getObjectByName(MAIN_LIGHT_NAME);
	if (!mainLight) return;
	mainLight.intensity = isDay ? MAIN_LIGHT_INTENSITY : 0.5;
	mainLight.castShadow = isDay;

	const ambientLight = scene.getObjectByName(AMBIENT_LIGHT_NAME);
	if (!ambientLight) return;
	ambientLight.intensity = isDay ? AMBIENT_LIGHT_INTENSITY : 5;

	const day_color = new THREE.Color(SKY_COLOR_DAY);
	const night_color = new THREE.Color(SKY_COLOR_NIGHT);
	scene.background = isDay ? day_color : night_color;
	scene.fog.color = isDay ? day_color : night_color;
}

export default initializeSceneEventHandlers;