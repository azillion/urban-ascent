import("../pkg/index.js").catch(console.error);

import { createRoot } from 'react-dom/client';
import React from 'react';
import { Provider } from 'react-redux'

import store from './ui/store/store';
import App from './ui/App';
import { animate } from './setup';

animate();

// Render your React component instead
const root = createRoot(document.getElementById('rootUI'));
root.render(
	<Provider store={store}>
		<App />
	</Provider>
);