import * as THREE from 'three';

function createGrid(size, divisions, color1 = 0x0000ff, color2 = 0xFFFFF7) {
	const gridHelper = new THREE.GridHelper(size, divisions, color1, color2);
	gridHelper.position.y = 0.1;
	gridHelper.name = 'grid';
	return gridHelper;
}

export { createGrid };