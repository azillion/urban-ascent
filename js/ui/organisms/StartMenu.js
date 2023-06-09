import React from 'react';
import { useSelector } from 'react-redux';

import NewGameMenu from './NewGameMenu';
import BaseStartMenu from './BaseStartMenu';
import { getNewGameMenuShown, getStartMenuShown } from '../store/startMenu';

export default function StartMenu() {
	const isBaseStartMenuShown = useSelector(getStartMenuShown);
	const isNewGameMenuShown = useSelector(getNewGameMenuShown);

	return (
		<>
			{isBaseStartMenuShown && (
				<BaseStartMenu />
			)}
			{isNewGameMenuShown && (
				<NewGameMenu />
			)}
		</>
	)
}
