import { generatePastelColor } from '../../utils/colors';
import { createCube } from '../components/cube';

function handleTerrainClick(intersections) {
	const terrain = intersections[0].object;
	const cube = createCube();
	cube.position.copy(intersections[0].point);
	cube.position.y = cube.geometry.parameters.height / 2 + 0.1;
	cube.material.color.set(generatePastelColor());
	terrain.add(cube);
}

export { handleTerrainClick };