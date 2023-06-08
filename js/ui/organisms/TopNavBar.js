import React from 'react';
import DayNightButton from './DayNightButton';

const styles = {
	topNavButtons: {
		pointerEvents: 'auto',
		display: 'flex',
		flexDirection: 'row',
		justifyContent: 'space-between',
		alignItems: 'center',
	},
};

export default function TopNavBar() {
	return (
		<div id="top-nav-bar" style={styles.topNavButtons}>
			<DayNightButton />
		</div>
	);
}
