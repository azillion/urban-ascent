import React from 'react';

export default function TextButton({handleOnClick, title, textStyle={}, buttonStyle={}, disabled=false}) {
	if (!title) return null;
	const [isHover, setIsHover] = React.useState(false);
	const [isPressed, setIsPressed] = React.useState(false);
	const handleOnMouseEnter = () => setIsHover(true);
	const handleOnMouseLeave = () => setIsHover(false);
	const handleOnMouseDown = () => setIsPressed(true);
	const handleOnMouseUp = () => setIsPressed(false);
	const buttonStylesCombined = {
		...styles.defaultButtonStyle,
		...buttonStyle,
		...(isHover ? styles.hoverButtonStyle : {}),
		...(isPressed ? styles.activeButtonStyle : {}),
		...(disabled ? styles.disabledButtonStyle : {}),
	};
	handleOnClick = disabled ? () => {} : handleOnClick;
	return (
		<button
			onClick={handleOnClick}
			onMouseEnter={handleOnMouseEnter}
            onMouseLeave={handleOnMouseLeave}
			onMouseDown={handleOnMouseDown}
			onMouseUp={handleOnMouseUp}
			style={buttonStylesCombined}
			disabled={disabled}
		>
			<span style={{...styles.defaultTextStyle, ...textStyle}}>{title}</span>
		</button>
	)
}

const styles = {
	defaultButtonStyle: {
		background: 'linear-gradient(45deg, #B2A4FF, #0080FF 100%)',
		border: '1px solid white',
		borderRadius: '5px',
		padding: '15px',
		margin: '10px',
	},
	hoverButtonStyle: {
		boxShadow: '0 2px 4px rgba(0, 0, 0, 0.2)',
		transform: 'scale(1.05)',
	},
	activeButtonStyle: {
		boxShadow: 'none',
		transform: 'scale(0.95)',
	},
	disabledButtonStyle: {
		opacity: 0.5,
		pointerEvents: 'none',
		transform: 'scale(1)',
		boxShadow: 'none',
	},
	defaultTextStyle: {
		color: 'white',
		fontFamily: 'sans-serif',
	},
};