import { globalEventManager } from '../../services/GlobalEventManager';
import { generatePastelColor } from '../../utils/colors';
import { Building, buildingsManager } from '../components/Building';
import { createCube } from '../components/cube';
import { TERRAIN_EVENTS } from '../components/terrain';

function initializeTerrainEventHandlers() {
	globalEventManager.registerEventListener(TERRAIN_EVENTS.TERRAIN_CLICK, handleTerrainClick);
}

function handleTerrainClick(intersections) {
	const terrain = intersections[0].object;
	const cube = createCube();
	cube.position.copy(intersections[0].point);
	cube.position.y = cube.geometry.parameters.height / 2 + 0.1;
	cube.material.color.set(generatePastelColor());
	const building = new Building(cube);
	buildingsManager.addBuilding(building);
	terrain.add(cube);
}

export default initializeTerrainEventHandlers;