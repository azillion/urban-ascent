import React from 'react';
import PausePlayButton from './PausePlayButton';

const styles = {
	bottomNavBarContainer: {
		pointerEvents: 'auto',
		width: '100%',
		height: '100%',
		borderRadius: '10px',
		background: 'linear-gradient(45deg, rgba(178, 164, 255, 0.5), rgba(0, 128, 255, 0.5))',
		border: '1px solid white',
	},
	row: {
		display: 'flex',
		flexDirection: 'row',
	},
};


export default function BottomNavBar() {
	return (
		<div id="bottom-nav-bar" style={styles.bottomNavBarContainer}>
			<div style={styles.row}>
				<PausePlayButton />
			</div>
		</div>	
	)
}
