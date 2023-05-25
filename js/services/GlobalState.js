import { globalEventManager } from './GlobalEventManager';
import {TOOL_MODES} from '../models/ToolMode';


class GlobalState {
	constructor() {
		this.globalEventManager = globalEventManager;
		this.toolMode = TOOL_MODES.PAN;
	}

	setToolMode(toolMode) {
		this.toolMode = toolMode;
		this.globalEventManager.dispatchEvent('toolModeChanged', toolMode);
	}
}

const globalState = new GlobalState();

export { globalState };