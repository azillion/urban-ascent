import React from 'react';
import { useSelector } from 'react-redux';

import StartMenu from './organisms/StartMenu';
import MainGameUILayout from './organisms/MainGameUILayout';
import { getGameLoaded } from './store/game';

export default function App() {
	const hasGameLoaded = useSelector(getGameLoaded);
	return (
		<div style={styles.root}>
			{hasGameLoaded ? <MainGameUILayout /> : <StartMenu />}
		</div>
	)
}

const styles = {
	root: {
		pointerEvents: 'none',
		width: '100%',
		height: '100%',
		display: 'flex',
		flexDirection: 'column',
		justifyContent: 'space-between',
	},
	topContainer: {
		display: 'flex',
		flexBasis: '10%',
		flexDirection: 'row',
		justifyContent: 'space-between',
		alignItems: 'center',
		padding: '0 10px 0 10px',
	},
	middleContainer: {
		display: 'flex',
		flexBasis: '75%',
		flexDirection: 'row',
		justifyContent: 'space-between',
		alignItems: 'stretch',
		margin: '10px',
	},
	bottomContainer: {
		display: 'flex',
		flexBasis: '15%',
		flexDirection: 'row',
		justifyContent: 'space-between',
		alignItems: 'stretch',
		margin: '10px',
	},
	title: {
		textAlign: 'center',
		color: 'tomato',
		fontFamily: 'sans-serif',
		background: '-webkit-linear-gradient(45deg, #B2A4FF, #0080FF 100%)',
		WebkitBackgroundClip: 'text',
		WebkitTextFillColor: 'transparent',
	},
};
