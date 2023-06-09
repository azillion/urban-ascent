import React from 'react';
import { useDispatch, useSelector } from 'react-redux';

// import TextButton from '../atoms/TextButton';
import {  getNewGameCityName, getNewGameMayorName, newGameCityNameSet, newGameMayorNameSet, newGameMenuShown } from '../store/startMenu';
import FullMenu from '../molecules/FullMenu';
import InputField from '../atoms/InputField';
import { gameHasLoaded } from '../store/game';
import TextButton from '../atoms/TextButton';
import { globalState } from '../../services/GlobalState';
// import { gameHasLoaded } from '../store/game';

export default function NewGameMenu() {
	const dispatch = useDispatch();
	const [isSlidingUp, setIsSlidingUp] = React.useState(false);
	const cityNameValue = useSelector(getNewGameCityName);
	const mayorNameValue = useSelector(getNewGameMayorName);

	const handleClose = () => {
		setIsSlidingUp(true);
		setTimeout(() => {
			setIsSlidingUp(false);
			dispatch(newGameMenuShown(false));
		}, 310);
	};
	const handleCityNameChange = (e) => {
		dispatch(newGameCityNameSet(e.target.value));
	};
	const handleMayorNameChange = (e) => {
		dispatch(newGameMayorNameSet(e.target.value));
	};
	let title = 'New Game';
	let subTitle = '';
	if (cityNameValue && cityNameValue.length > 0) {
		title = `${cityNameValue}`;
	}
	if (mayorNameValue && mayorNameValue.length > 0) {
		subTitle = `led by ${mayorNameValue}`;
	}
	const isDisabled = !cityNameValue || cityNameValue.length === 0 || !mayorNameValue || mayorNameValue.length === 0;
	const handleStartGame = () => {
		if (isDisabled) {
			return;
		}
		setIsSlidingUp(true);
		setTimeout(() => {
			setIsSlidingUp(false);
			dispatch(gameHasLoaded(true));
			globalState.gameHasLoaded = true;
			dispatch(newGameMenuShown(false));
		}, 310);
	};
	return (
		<FullMenu isSlidingUp={isSlidingUp} setIsSlidingUp={setIsSlidingUp} closeCallback={handleClose} menuStyle={styles.newGameMenu}>
			<h1 style={styles.title}>{title}</h1>
			{ subTitle && <h2 style={styles.subTitle}>{subTitle}</h2> }
			{ !subTitle && <div style={{height: '1.5rem'}} /> }
			<InputField label='City Name:' inputValue={cityNameValue} onChange={handleCityNameChange} />
			<InputField label='Mayor:' inputValue={mayorNameValue} onChange={handleMayorNameChange} />
			<TextButton title='Start Game' handleOnClick={handleStartGame} disabled={isDisabled} buttonStyle={{ marginTop: '20px', marginBottom: '20px' }} />
		</FullMenu>
	)
}

const styles = {
	newGameMenu: {
		width: '60%',
		height: '40%',
	},
	title: {
		textAlign: 'center',
		color: 'white',
		fontFamily: 'sans-serif',
		background: '-webkit-linear-gradient(45deg, #FFB2A4, #FF0080)',
		WebkitBackgroundClip: 'text',
		WebkitTextFillColor: 'transparent',
		marginTop: '0',
		marginBottom: '0',
	},
	subTitle: {
		textAlign: 'center',
		color: 'white',
		fontFamily: 'sans-serif',
		marginTop: '5px',
		marginBottom: '5px',
	},
};
