import React from 'react';

const styles = {
	infoBarContainer: {
		pointerEvents: 'auto',
		width: '10%',
		height: '100%',
		borderRadius: '10px',
		background: 'linear-gradient(45deg, rgba(178, 164, 255, 0.5), rgba(0, 128, 255, 0.5))',
		border: '1px solid white',
	},
};

export default function LeftInfoBar() {
	return (
		<div id="left-info-bar" style={styles.infoBarContainer}>
		</div>
	);
}
