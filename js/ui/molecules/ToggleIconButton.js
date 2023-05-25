import React from 'react'
import IconButton from '../atoms/IconButton';

export default function ToggleIconButton({
	handleOnClick,
	isOnDefault = true,
	iconOn,
	iconOff,
	iconOnStyle = {},
	iconOffStyle = {},
	buttonStyle = {},
	iconProps = {},
}) {
	const [isOn, setIsOn] = React.useState(isOnDefault);
	const [icon, setIcon] = React.useState(isOnDefault ? iconOn : iconOff);
	const [iconStyle, setIconStyle] = React.useState(isOnDefault ? iconOnStyle : iconOffStyle);
	if (!handleOnClick) handleOnClick = () => console.warn('No onClick handler provided for IconButton', icon);

	const handleOnClickAndToggle = () => {
		try {
			handleOnClick(!isOn);
			setIsOn(!isOn);
			setIcon(isOn ? iconOff : iconOn);
			setIconStyle(isOn ? iconOffStyle : iconOnStyle);
		} catch (error) {
			console.error(error);
		}
	};
	if (!icon) return null;
	
	return (
		<IconButton 
			icon={icon}
			handleOnClick={handleOnClickAndToggle}
			iconStyle={iconStyle}
			buttonStyle={buttonStyle}
			iconProps={iconProps}
		/>
	);
}
