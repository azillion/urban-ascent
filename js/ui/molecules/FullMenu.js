import React from 'react';
import { icon } from '@fortawesome/fontawesome-svg-core/import.macro';

import IconButton from '../atoms/IconButton';
import SlideUpContainer from './SlideUpContainer';

const closeIcon = icon({ name: 'close', style: 'solid' })

function CloseButton({handleOnClick}) {
	return (
		<IconButton handleOnClick={handleOnClick} icon={closeIcon} buttonStyle={styles.closeButton} iconStyle={styles.closeIcon} />
	);
}

export default function FullMenu({children, containerStyle = {}, menuStyle={}, canClose = true, closeCallback = () => {}, isSlidingUp = false, setIsSlidingUp = () => {}}) {
	const containerStyles = {
		...styles.container,
		...containerStyle,
	};
	const menuStyles = {
		...styles.menu,
		...menuStyle,
	};
	const closeCallbackWrapper = () => {
		setIsSlidingUp(true);
		setTimeout(() => {
			setIsSlidingUp(false);
			closeCallback();
		}, 310);
	};
	return (
		<SlideUpContainer isSlidingUp={isSlidingUp}>
			<div style={containerStyles}>
				<div style={menuStyles}>
					{ canClose && (
						<div style={styles.navRow}>
							<CloseButton handleOnClick={closeCallbackWrapper} />
						</div>
					)}
					{children}
				</div>
			</div>
		</SlideUpContainer>
	);
}

const styles = {
	container: {
		pointerEvents: 'none',
		display: 'flex',
		flexDirection: 'column',
		justifyContent: 'center',
		alignItems: 'center',
		width: '100%',
		height: '100%',
		transition: 'transform 0.3s ease-out'
	},
	menu: {
		pointerEvents: 'auto',
		display: 'flex',
		flexDirection: 'column',
		justifyContent: 'flex-start',
		alignItems: 'center',
		width: '30%',
		height: '60%',
		borderRadius: '10px',
		background: 'linear-gradient(45deg, rgba(178, 164, 255, 0.9), rgba(0, 128, 255, 0.9))',
		border: '1px solid white',
	},
	navRow: {
		width: '100%',
		display: 'flex',
		flexDirection: 'row',
		justifyContent: 'flex-end',
		alignItems: 'center',
	},
	closeButton: {
		width: '30px',
		height: '30px',
		borderRadius: '50%',
		background: 'linear-gradient(45deg, rgba(178, 164, 255, 0.8), rgba(0, 128, 255, 0.8))',
		border: '1px solid white',
		margin: '10px',
	},
	closeIcon: {
		color: 'tomato',
	},
};