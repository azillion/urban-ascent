import { world } from '..';
import { TOOL_MODES } from '../../models/ToolMode';
import { globalEventManager } from '../../services/GlobalEventManager';
import { globalState } from '../../services/GlobalState';
import { NODE_EVENTS } from '../components/paths/node';
import { WORLD_EVENTS } from '../world';

function initializeRoadCreationEventHandlers() {
	globalEventManager.registerEventListener(NODE_EVENTS.NODE_MOUSE_DOWN, handleMouseDown);
	globalEventManager.registerEventListener(NODE_EVENTS.NODE_MOUSE_MOVE, handleMouseMove);
	globalEventManager.registerEventListener(NODE_EVENTS.NODE_MOUSE_UP, handleMouseUp);
	globalEventManager.registerEventListener(WORLD_EVENTS.TOOL_MODE_CHANGED, handleToolModeChanged);
}

function handleToolModeChanged(toolMode) {
	if (toolMode === TOOL_MODES.PAN) {
		world.controlsPanningOn();
		world.pathCreationNode.visible = false;
		world.grid.visible = false;
	}
	if (toolMode === TOOL_MODES.BUILD_STRAIGHT_ROAD) {
		world.controlsPanningOff();
		world.pathCreationNode.visible = true;
		world.grid.visible = true;
	}
}

function handleMouseDown(intersections) {
	if (!intersections.length) return;
	if (globalState.toolMode !== TOOL_MODES.BUILD_STRAIGHT_ROAD) return;
	const node = world.pathCreationNode;
	if (!node) return;
	// world.pathCreationNode.visible = false;
	// let terrain;
	// for (const intersect of intersections) {
	// 	if (intersect.object.name === 'terrain') {
	// 		terrain = intersect;
	// 	}
	// }
	// if (!terrain) return;
	// const pos = terrain.point;
	// pos.y = node.position.y;
	// const gridPosition = snapToGrid(pos);


	// // TODO check collisions with other nodes and buildings
	// world.isDrawing = true;
	// world.pathStart
	// const node = intersections[0].object;
	// if (node.name !== 'node') return;
	// node.parent.roadCreationManager.onMouseDown(intersections);
}


// TODO two functions, one for mouse move before mouse down, one for during
function handleMouseMove(intersections) {
	if (globalState.toolMode !== TOOL_MODES.BUILD_STRAIGHT_ROAD) return;
	// if (!world.isDrawing) return;
	if (!world.pathCreationNode) return;
	const node = world.pathCreationNode;
	let terrain;
	for (const intersect of intersections) {
		if (intersect.object.name === 'terrain') {
			terrain = intersect;
		}
	}
	if (!terrain) return;
	const pos = terrain.point;
	pos.y = node.position.y;
	const gridPosition = snapToGrid(pos);
	node.position.copy(gridPosition);
}

function handleMouseUp(intersections) {
	// if (globalState.toolMode !== TOOL_MODES.BUILD_STRAIGHT_ROAD) return;
	// if (!world.isDrawing) return;
	// if (!intersections.length) return;
	// world.isDrawing = false;
	// world.pathStartPoint = null;
	// TODO create road in wasm
	// const node = intersections[0].object;
	// if (node.name !== 'node') return;
	// node.parent.roadCreationManager.onMouseUp(intersections);
}

function snapToGrid(intersectPoint) {
    const gridPosition = intersectPoint.clone();
	gridPosition.x = Math.round(gridPosition.x);
	gridPosition.z = Math.round(gridPosition.z);
    gridPosition.y = 0.1;

    return gridPosition;
}

export default initializeRoadCreationEventHandlers;