import { createRoot } from 'react-dom/client';
import React from 'react';
import { Provider } from 'react-redux'

import store from './store';
import App from './App';

export default function initializeUI(container) {
	const root = createRoot(container);
	root.render(
		<Provider store={store}>
			<App />
		</Provider>
	);
}