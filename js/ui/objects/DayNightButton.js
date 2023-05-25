import React from 'react';
import { icon } from '@fortawesome/fontawesome-svg-core/import.macro';

import ToggleIconButton from '../molecules/ToggleIconButton';
import { globalEventManager } from '../../services/GlobalEventManager';
import { WORLD_EVENTS } from '../../world/world';

const buttonStyle = {
	width: '40px',
	height: '40px',
	borderRadius: '4px',
	backgroundColor: 'grey',
	border: 'none',
	outline: 'none',
};

export default function DayNightButton() {
	const handleOnClick = (isDay) => {
		globalEventManager.dispatchEvent(WORLD_EVENTS.TOGGLE_DAY_NIGHT, isDay);
	};

	return (
		<ToggleIconButton 
			iconOff={icon({name: 'moon', style: 'solid'})}
			iconOn={icon({name: 'sun', style: 'solid'})}
			handleOnClick={handleOnClick}
			iconOffStyle={{color: 'black'}}
			iconOnStyle={{color: 'gold'}}
			buttonStyle={buttonStyle}
		/>
	);
}
