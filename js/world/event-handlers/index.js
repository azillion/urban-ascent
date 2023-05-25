import initializeBuildingEventHandlers from './building';
import initializeTerrainEventHandlers from './terrain';

function initializeWorldEventHandlers() {
	// Event handlers for the in-game world
	initializeTerrainEventHandlers();
	initializeBuildingEventHandlers();
}

export { initializeWorldEventHandlers };