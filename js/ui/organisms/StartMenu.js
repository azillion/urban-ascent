import React from 'react';
import { useDispatch } from 'react-redux';

import TextButton from '../atoms/TextButton';
import { gameHasLoaded } from '../store/game';
import { globalState } from '../../services/GlobalState';

export default function StartMenu() {
	const dispatch = useDispatch();
	const [isSlidingUp, setIsSlidingUp] = React.useState(false);
	const containerStyles = {
		...styles.startMenuContainer,
		...(isSlidingUp && styles.startMenuContainerSlideUp),
	};
	const handleStartGame = () => {
		setIsSlidingUp(true);
		setTimeout(() => {
			setIsSlidingUp(false);
			dispatch(gameHasLoaded(true));
			globalState.gameHasLoaded = true;
		}, 310);
	};
	const handleStartNewGame = () => {};

	return (
		<div style={containerStyles}>
			<div id='start-menu' style={styles.startMenu}>
				<h1 style={styles.title}>Urban Ascent</h1>
				<TextButton title='New Game' handleOnClick={handleStartNewGame} />
				<TextButton title='Load Game' handleOnClick={handleStartGame} />
				<TextButton title='Options' handleOnClick={() => {}} disabled />
			</div>
		</div>
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
		// background: '-webkit-linear-gradient(45deg, #B2A4FF, #0080FF 100%)',
		WebkitBackgroundClip: 'text',
		WebkitTextFillColor: 'transparent',
	},
};
