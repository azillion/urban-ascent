import React from 'react';
import { icon } from '@fortawesome/fontawesome-svg-core/import.macro';

import ToggleIconButton from '../molecules/ToggleIconButton';
import { globalState } from '../../services/GlobalState';
import { TOOL_MODES } from '../../models/ToolMode';

const roadIcon = icon({ name: 'road', style: 'solid' });

export default function StraightRoadButton() {
	const [buttonStyle, setButtonStyle] = React.useState({ background: 'linear-gradient(45deg, #B2A4FF, #0080FF 100%)' });
	const handleOnClick = (isOn) => {
		setButtonStyle({ background: isOn ? 'linear-gradient(45deg, #FFB2A4, #FF0080)' : 'linear-gradient(45deg, #B2A4FF, #0080FF 100%)' });
		globalState.setToolMode(isOn ? TOOL_MODES.BUILD_STRAIGHT_ROAD : TOOL_MODES.PAN);
	};
	return (
		<ToggleIconButton
			handleOnClick={handleOnClick}
			isOnDefault={false}
			iconOff={roadIcon}
			iconOn={roadIcon}
			iconOffStyle={{ color: 'lightgray' }}
			iconOnStyle={{ color: 'white' }}
			buttonStyle={buttonStyle}
		/>
	)
}
