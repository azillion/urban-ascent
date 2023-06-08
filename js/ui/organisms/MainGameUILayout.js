import React, { useEffect } from 'react';
import { useSelector } from 'react-redux';

import TopNavBar from './TopNavBar';
import LeftInfoBar from './LeftInfoBar';
import BottomNavBar from './BottomNavBar';
import { getGameLoaded } from '../store/game';

export default function MainGameUILayout() {
	const hasGameLoaded = useSelector(getGameLoaded);
	const [isShown, setIsShown] = React.useState(false);
	useEffect(() => {
		if (hasGameLoaded) {
			setTimeout(() => {
				setIsShown(true);
			}, 1);
		} else {
			setIsShown(false);
		}
	}, [hasGameLoaded]);
	return (
		<>
			<div style={{...styles.topContainer, ...(isShown && styles.shown)}}>
				<h1 style={styles.title}>Urban Ascent</h1>
				<TopNavBar />
			</div>
			<div style={{...styles.middleContainer, ...(isShown && styles.shown)}}>
				<LeftInfoBar />
			</div>
			<div style={{...styles.bottomContainer, ...(isShown && styles.shown)}}>
				<BottomNavBar />
			</div>
		</>
	);
}


const styles = {
	shown: {
		opacity: 1,
		transition: 'opacity 0.3s ease-in',
	},
	topContainer: {
		display: 'flex',
		flexBasis: '10%',
		flexDirection: 'row',
		justifyContent: 'space-between',
		alignItems: 'center',
		padding: '0 10px 0 10px',
		opacity: 0,
	},
	middleContainer: {
		display: 'flex',
		flexBasis: '75%',
		flexDirection: 'row',
		justifyContent: 'space-between',
		alignItems: 'stretch',
		margin: '10px',
		opacity: 0,
	},
	bottomContainer: {
		display: 'flex',
		flexBasis: '15%',
		flexDirection: 'row',
		justifyContent: 'space-between',
		alignItems: 'stretch',
		margin: '10px',
		opacity: 0,
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
