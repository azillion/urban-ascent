import React from 'react'
import { icon } from '@fortawesome/fontawesome-svg-core/import.macro';

import ToggleIconButton from '../molecules/ToggleIconButton'
import { globalEventManager } from '../../services/GlobalEventManager';
import { WORLD_EVENTS } from '../../world/world';

export default function PausePlayButton() {
	const handleOnClick = (isPlaying) => {
		globalEventManager.dispatchEvent(WORLD_EVENTS.TOGGLE_PAUSE_PLAY, isPlaying);
	};

	return (
		<ToggleIconButton
			iconOff={icon({ name: 'pause', style: 'solid' })}
			iconOn={icon({ name: 'play', style: 'solid' })}
			handleOnClick={handleOnClick}
			iconOffStyle={{ color: 'tomato' }}
			iconOnStyle={{ color: 'white' }}
			buttonStyle={{ background: 'linear-gradient(45deg, #B2A4FF, #0080FF 100%)' }}
		/>
	);
}
