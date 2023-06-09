import { combineReducers } from '@reduxjs/toolkit';

import bottomToolbarReducer from './bottomToolbar';
import gameReducer from './game';
import startMenuReducer from './startMenu';

export default combineReducers({
	bottomToolbar: bottomToolbarReducer,
	game: gameReducer,
	startMenu: startMenuReducer,
});