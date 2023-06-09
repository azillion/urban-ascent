import React from 'react';
import { useDispatch } from 'react-redux';

import TextButton from '../atoms/TextButton';
import { globalState } from '../../services/GlobalState';
import {  newGameMenuShown } from '../store/startMenu';
import FullMenu from '../molecules/FullMenu';
import { gameHasLoaded } from '../store/game';

export default function BaseMenu() {
	const dispatch = useDispatch();
	const [isSlidingUp, setIsSlidingUp] = React.useState(false);

	const handleNewGameClick = () => {
		setIsSlidingUp(true);
		setTimeout(() => {
			setIsSlidingUp(false);
			dispatch(newGameMenuShown(true));
		}, 310);
	};
	const handleLoadGameClick = () => {
		setIsSlidingUp(true);
		setTimeout(() => {
			setIsSlidingUp(false);
			dispatch(gameHasLoaded(true));
			globalState.gameHasLoaded = true;
		}, 310);
	};
	let hasContinuedGame = false;
	if (globalState.gameManager) {
		hasContinuedGame = globalState.gameManager.hasContinuedGame();
	}
	return (
		<FullMenu isSlidingUp={isSlidingUp} setIsSlidingUp={setIsSlidingUp} canClose={false}>
			<h1 style={styles.title}>Urban Ascent</h1>
			<TextButton title='New Game' handleOnClick={handleNewGameClick} />
			<TextButton title='Continue Game' handleOnClick={handleLoadGameClick} />
			<TextButton title='Load Previous Save' handleOnClick={handleLoadGameClick} disabled={!hasContinuedGame} />
			<TextButton title='Upload Save' handleOnClick={handleLoadGameClick} disabled />
			<TextButton title='Options' handleOnClick={() => {}} disabled />
		</FullMenu>
	)
}

const styles = {
	startMenuContainer: {
		pointerEvents: 'none',
		display: 'flex',
		flexDirection: 'column',
		justifyContent: 'center',
		alignItems: 'center',
		width: '100%',
		height: '100%',
		transition: 'transform 0.3s ease-out'
	},
	startMenuContainerSlideUp: {
		transform: 'translateY(-100%)',
	},
	startMenu: {
		pointerEvents: 'auto',
		display: 'flex',
		flexDirection: 'column',
		justifyContent: 'flex-start',
		alignItems: 'center',
		width: '30%',
		height: '60%',
		borderRadius: '10px',
		background: 'linear-gradient(45deg, rgba(178, 164, 255, 0.8), rgba(0, 128, 255, 0.8))',
		border: '1px solid white',
	},
	title: {
		textAlign: 'center',
		color: 'white',
		fontFamily: 'sans-serif',
		background: '-webkit-linear-gradient(45deg, #FFB2A4, #FF0080)',
		WebkitBackgroundClip: 'text',
		WebkitTextFillColor: 'transparent',
	},
};
