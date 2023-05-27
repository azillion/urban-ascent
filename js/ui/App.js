import React from 'react';
import DayNightButton from './objects/DayNightButton';
import PausePlayButton from './objects/PausePlayButton';

export default function App() {
  return (
	<div style={styles.root}>
		<h1 style={styles.h1}>Urban Ascent</h1>
		<div style={styles.topNavButtons}>
			<DayNightButton />
			<PausePlayButton />
		</div>
	</div>
  )
}

const styles = {
	root: {
		textAlign: 'center',
		pointerEvents: 'auto',
		display: 'flex',
		flexDirection: 'row',
		justifyContent: 'space-between',
		alignItems: 'center',
		padding: '0 10px 0 10px',
	},
	h1: {
		color: 'tomato',
		fontFamily: 'sans-serif',
		background: '-webkit-linear-gradient(45deg, #B2A4FF, #0080FF 100%)',
		WebkitBackgroundClip: 'text',
		WebkitTextFillColor: 'transparent',
	},
	topNavButtons: {
		display: 'flex',
		flexDirection: 'row',
		justifyContent: 'space-between',
		alignItems: 'center',
	},
}