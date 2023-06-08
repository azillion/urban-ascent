import { globalEventManager } from '../../services/GlobalEventManager';
import { generatePastelColor } from '../../utils/colors';
import { Building, buildingsManager } from '../components/Building';
import { createCube } from '../components/cube';
import { TERRAIN_EVENTS } from '../components/terrain';
import { world } from '../';
import { globalState } from '../../services/GlobalState';
import { TOOL_MODES } from '../../models/ToolMode';

function initializeTerrainEventHandlers() {
	globalEventManager.registerEventListener(TERRAIN_EVENTS.TERRAIN_MOUSE_UP, handleTerrainMouseUp);
}

function handleTerrainMouseUp(intersections) {
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

	if (!terrain) return;

	switch (globalState.toolMode) {
		case TOOL_MODES.PLACE_BUILDING:
			placeBuilding(terrain);
			break;
		default:
			break;
	}
}

function placeBuilding(terrain) {
	const cube = createCube();
	cube.name = 'building';
	const pos = terrain.point;
	pos.round();
	console.log(pos);
	cube.position.copy(pos);
	
	cube.position.y = cube.geometry.parameters.height / 2 + 0.1;
	cube.material.color.set(generatePastelColor());
	const building = new Building(cube);
	buildingsManager.addBuilding(building);
	terrain.object.add(cube);
}

export default initializeTerrainEventHandlers;