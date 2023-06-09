import { createSelector, createSlice } from '@reduxjs/toolkit';

const slice = createSlice({
	name: 'game',
	initialState: {
		gameLoaded: false,
	},
	reducers: {
		gameHasLoaded: (game, action) => {
			game.gameLoaded = action.payload;
		},
	}
});

// Action creators are generated for each case reducer function
export const { gameHasLoaded } = slice.actions;

export const getGameLoaded = createSelector(
	(state) => state.game,
	(game) => game.gameLoaded
);

export default slice.reducer;