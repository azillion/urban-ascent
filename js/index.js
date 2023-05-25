import("../pkg/index.js").catch(console.error);

import initializeUI from './ui';
import initializeWorld from './world';

async function main() {
	const root = document.getElementById('root');
	const rootUI = document.getElementById('rootUI');

	await initializeWorld(root).catch((err) => {
		console.error(err);
	});
	initializeUI(rootUI);
}

await main();
