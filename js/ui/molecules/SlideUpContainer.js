import React from 'react'

export default function SlideUpContainer({ children, isSlidingUp = false}) {

	const containerStyles = {
		...styles.slideUpContainer,
		...(isSlidingUp && styles.containerSlideUp),
	};
	return (
		<div style={containerStyles}>
			{children}
		</div>
	);
}



const styles = {
	slideUpContainer: {
		pointerEvents: 'none',
		display: 'flex',
		flexDirection: 'column',
		justifyContent: 'center',
		alignItems: 'center',
		width: '100%',
		height: '100%',
		transition: 'transform 0.3s ease-out'
	},
	containerSlideUp: {
		transform: 'translateY(-100%)',
	},
};
