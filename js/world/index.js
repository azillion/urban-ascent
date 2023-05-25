import {World} from './world';

let world;

export default async function initializeWorld(container) {
	world = new World(container);

	await world.init();
	
	world.start();
}

export { world };