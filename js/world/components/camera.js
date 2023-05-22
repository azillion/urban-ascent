import * as THREE from 'three';

function createCamera() {
	const camera = new THREE.PerspectiveCamera(
		75, // fov = field of view
		window.innerWidth / window.innerHeight, // aspect ratio
		0.1, // near plane
		1000 // far plane
	);

	camera.position.z = 5;
	return camera;
}

export { createCamera };