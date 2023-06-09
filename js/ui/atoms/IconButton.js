import React from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

export default function IconButton({
		handleOnClick,
		icon, 
		iconStyle={}, 
		iconProps={}, 
		buttonStyle={}, 
		disabled=false
	}) {
	if (!icon) return null;
	if (!handleOnClick) handleOnClick = () => {console.warn('No onClick handler provided for IconButton', icon)};
	const [isHover, setIsHover] = React.useState(false);
	const [isPressed, setIsPressed] = React.useState(false);
	const handleOnMouseEnter = () => setIsHover(true);
	const handleOnMouseLeave = () => setIsHover(false);
	const handleOnMouseDown = () => setIsPressed(true);
	const handleOnMouseUp = () => setIsPressed(false);
	const buttonStylesCombined = {
		...buttonStyle,
		...(isHover ? styles.hoverButtonStyle : {}),
		...(isPressed ? styles.activeButtonStyle : {}),
		...(disabled ? styles.disabledButtonStyle : {}),
	};
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
			<FontAwesomeIcon icon={icon} style={iconStyle} {...iconProps} />
		</button>
	)
}

const styles = {
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
};
