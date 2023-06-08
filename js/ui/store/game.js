import { createSelector, createSlice } from '@reduxjs/toolkit';

const slice = createSlice({
	name: 'game',
	initialState: {
		gameLoaded: false,
		savedGameTitle: '',
		savedGames: [],
	},
	reducers: {
		saveGameTitleSet: (game, action) => {
			game.savedGameTitle = action.payload;
		},
		gameHasLoaded: (game, action) => {
			game.gameLoaded = action.payload;
		},
		savedGamesListLoaded: (game, action) => {
			game.savedGames = action.payload;
		},
	}
});

// Action creators are generated for each case reducer function
export const { saveGameTitleSet, gameHasLoaded } = slice.actions;

export const getGameLoaded = createSelector(
	(state) => state.game,
	(game) => game.gameLoaded
);

export default slice.reducer;