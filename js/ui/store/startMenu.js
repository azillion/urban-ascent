import { createSelector, createSlice } from '@reduxjs/toolkit';

const slice = createSlice({
	name: 'startMenu',
	initialState: {
		menuShown: true,
		newGameMenuShown: false,
		savedGameTitle: '',
		savedGames: [],
		newGameCityName: '',
		newGameMayorName: '',
	},
	reducers: {
		menuShown: (startMenu, action) => {
			startMenu.menuShown = action.payload;
			startMenu.newGameMenuShown = !action.payload;
		},
		newGameMenuShown: (startMenu, action) => {
			startMenu.newGameMenuShown = action.payload;
			startMenu.menuShown = !action.payload;
		},
		saveGameTitleSet: (startMenu, action) => {
			startMenu.savedGameTitle = action.payload;
		},
		savedGamesListLoaded: (startMenu, action) => {
			startMenu.savedGames = action.payload;
		},
		newGameCityNameSet: (startMenu, action) => {
			startMenu.newGameCityName = action.payload;
		},
		newGameMayorNameSet: (startMenu, action) => {
			startMenu.newGameMayorName = action.payload;
		}
	}
});

// Action creators are generated for each case reducer function
export const { menuShown, newGameMenuShown, saveGameTitleSet, savedGamesListLoaded, newGameCityNameSet, newGameMayorNameSet } = slice.actions;


export const getStartMenuShown = createSelector(
	(state) => state.startMenu,
	(startMenu) => startMenu.menuShown
);

export const getNewGameMenuShown = createSelector(
	(state) => state.startMenu,
	(startMenu) => startMenu.newGameMenuShown
);

export const getNewGameCityName = createSelector(
	(state) => state.startMenu,
	(startMenu) => startMenu.newGameCityName
);

export const getNewGameMayorName = createSelector(
	(state) => state.startMenu,
	(startMenu) => startMenu.newGameMayorName
);

export default slice.reducer;