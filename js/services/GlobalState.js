import { globalEventManager } from './GlobalEventManager';
import {TOOL_MODES} from '../models/ToolMode';
import { WORLD_EVENTS } from '../world/world';


class GlobalState {
	constructor() {
		this.toolMode = TOOL_MODES.PAN;
		this._gameHasLoaded = false;
	}

	setToolMode(toolMode) {
		this.toolMode = toolMode;
		globalEventManager.dispatchEvent(WORLD_EVENTS.TOOL_MODE_CHANGED, toolMode);
	}

	get gameHasLoaded() {
		return this._gameHasLoaded;
	}

	set gameHasLoaded(gameHasLoaded = false) {
		if (typeof gameHasLoaded !== 'boolean') throw new Error('gameHasLoaded must be a boolean');
		this._gameHasLoaded = gameHasLoaded;
		globalEventManager.dispatchEvent(WORLD_EVENTS.GAME_HAS_LOADED, gameHasLoaded);
	}
}

const globalState = new GlobalState();

export { globalState };