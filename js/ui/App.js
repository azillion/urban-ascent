import React from 'react'

export default function App() {
  return (
	<div style={styles.root}>
		<h1 style={styles.h1}>Urban Ascent</h1>
		<button>Hello</button>
	</div>
  )
}

const styles = {
	root: {
		textAlign: 'center',
		pointerEvents: 'auto'
	},
	h1: {
		color: 'tomato',
		fontFamily: 'sans-serif',
		background: '-webkit-linear-gradient(45deg, #B2A4FF, #0080FF 100%)',
		WebkitBackgroundClip: 'text',
		WebkitTextFillColor: 'transparent',
	}
}