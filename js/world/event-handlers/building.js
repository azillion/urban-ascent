import { globalEventManager } from '../../services/GlobalEventManager';
import { buildingsManager, BUILDING_EVENTS } from '../components/Building';
import { world } from '../';


function initializeBuildingEventHandlers() {
	globalEventManager.registerEventListener(BUILDING_EVENTS.BUILDING_MOUSE_DOWN, handleMouseDown);
	globalEventManager.registerEventListener(BUILDING_EVENTS.BUILDING_MOUSE_MOVE, handleMouseMove);
	globalEventManager.registerEventListener(BUILDING_EVENTS.BUILDING_MOUSE_UP, handleMouseUp);
}

function handleMouseDown(intersections) {
	const cube = intersections[0].object;
	if (cube.name !== 'building') return;
	const building = buildingsManager.getBuilding(cube.id);
	if (!building) return;
	world.controls.enabled = false;
	world.isDragging = true;
	world.draggedEntity = building;
	building.dragManager.onMouseDown(intersections);
}

function handleMouseMove(intersections) {
	if (!world.isDragging) return;
	if (!world.draggedEntity) return;
	const building = world.draggedEntity;
	building.dragManager.onMouseMove(intersections);
}

function handleMouseUp(intersections) {
	const building = world.draggedEntity;
	if (!building) return;
	building.dragManager.onMouseUp(intersections);
	world.controls.enabled = true;
	world.isDragging = false;
	world.draggedEntity = null;
}

export default initializeBuildingEventHandlers;