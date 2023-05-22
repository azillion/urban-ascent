import {World} from './world';

export default async function initializeWorld(container) {
	const world = new World(container);

	await world.init();
	
	world.start();
}