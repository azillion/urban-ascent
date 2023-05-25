import { globalEventManager } from '../../services/GlobalEventManager';
import { buildingsManager, BUILDING_EVENTS } from '../components/Building';


function initializeBuildingEventHandlers() {
	globalEventManager.registerEventListener(BUILDING_EVENTS.BUILDING_MOUSE_DOWN, handleMouseDown);
	globalEventManager.registerEventListener(BUILDING_EVENTS.BUILDING_MOUSE_MOVE, handleMouseMove);
	globalEventManager.registerEventListener(BUILDING_EVENTS.BUILDING_MOUSE_UP, handleMouseUp);
}

function handleMouseDown(intersections) {
	const cube = intersections[0].object;
	const building = buildingsManager.getBuilding(cube.id);
	building.dragManager.onMouseDown(intersections);
}

function handleMouseMove(intersections) {
	const cube = intersections[0].object;
	const building = buildingsManager.getBuilding(cube.id);
	building.dragManager.onMouseMove(intersections);
}

function handleMouseUp(intersections) {
	const cube = intersections[0].object;
	const building = buildingsManager.getBuilding(cube.id);
	building.dragManager.onMouseUp(intersections);
}

export default initializeBuildingEventHandlers;