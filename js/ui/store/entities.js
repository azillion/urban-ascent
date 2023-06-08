import { combineReducers } from '@reduxjs/toolkit';

import bottomToolbarReducer from './bottomToolbar';
import gameReducer from './game';

export default combineReducers({
	game: gameReducer,
	bottomToolbar: bottomToolbarReducer,
});