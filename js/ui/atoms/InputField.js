import React from 'react';

export default function InputField({label, inputValue, onChange, onEnter=() => {}, onEscape=() => {}, placeholder='', maxLength=100}) {
	return (
		<div style={styles.inputContainer}>
			<label style={styles.label}>{label}</label>
			<input style={styles.input} placeholder={placeholder} maxLength={maxLength} value={inputValue} onChange={onChange} onKeyDown={(e) => {
				if (e.key === 'Enter') {
					onEnter();
				} else if (e.key === 'Escape') {
					onEscape();
				}
			}} />
		</div>
	);
}


const styles = {
	inputContainer: {
		width: '300px',
		height: '40px',
		display: 'flex',
		flexDirection: 'row',
		justifyContent: 'center',
		alignItems: 'center',
	},
	label: {
		marginRight: '10px',
		color: 'white',
		minWidth: 'max-content',
	},
	input: {
	},
};