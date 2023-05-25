import React from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

export default function IconButton({handleOnClick, icon, iconStyle={}, iconProps={}, buttonStyle={}}) {
	if (!icon) return null;
	if (!handleOnClick) handleOnClick = () => {console.warn('No onClick handler provided for IconButton', icon)};
	return (
		<button onClick={handleOnClick} style={buttonStyle}>
			<FontAwesomeIcon icon={icon} style={iconStyle} {...iconProps} />
		</button>
	)
}
