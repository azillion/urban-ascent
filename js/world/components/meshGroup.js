import {
SphereBufferGeometry,
Group,
MathUtils,
Mesh,
MeshStandardMaterial,
} from 'three';

function createMeshGroup() {
	const group = new Group();

	group.tick = (delta) => {};

	return group;
}

export { createMeshGroup };