import initializeBuildingEventHandlers from './building';
import initializeSceneEventHandlers from './scene';
import initializeTerrainEventHandlers from './terrain';

function initializeWorldEventHandlers() {
	// Event handlers for the in-game world
	initializeTerrainEventHandlers();
	initializeBuildingEventHandlers();
	initializeSceneEventHandlers();
}

export { initializeWorldEventHandlers };