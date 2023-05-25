import { globalEventManager } from '../../services/GlobalEventManager';
import { TERRAIN_EVENTS } from '../components/terrain';
import { handleTerrainClick } from './terrain';

function initializeWorldEventHandlers() {
	// Event handlers for the in-game world
	globalEventManager.registerEventListener(TERRAIN_EVENTS.TERRAIN_CLICK, handleTerrainClick);

}

export { initializeWorldEventHandlers };