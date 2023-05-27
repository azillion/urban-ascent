import { globalEventManager } from '../../services/GlobalEventManager';
import { generatePastelColor } from '../../utils/colors';
import { Building, buildingsManager } from '../components/Building';
import { createCube } from '../components/cube';
import { TERRAIN_EVENTS } from '../components/terrain';
import { world } from '../';

function initializeTerrainEventHandlers() {
	globalEventManager.registerEventListener(TERRAIN_EVENTS.TERRAIN_CLICK, handleTerrainClick);
}

function handleTerrainClick(intersections) {
	let terrain;
	if (world.isDragging) return;
	for (const intersect of intersections) {
		if (intersect.object.name === 'terrain') {
			terrain = intersect;
		}
		if (intersect.object.name === 'building') {
			return;
		}
	}

	const cube = createCube();
	cube.name = 'building';
	cube.position.copy(terrain.point);
	cube.position.y = cube.geometry.parameters.height / 2 + 0.1;
	cube.material.color.set(generatePastelColor());
	const building = new Building(cube);
	buildingsManager.addBuilding(building);
	terrain.object.add(cube);
}

export default initializeTerrainEventHandlers;