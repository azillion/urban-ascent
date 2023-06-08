import * as THREE from 'three';

function createRenderer() {
	const renderer = new THREE.WebGLRenderer({ antialias: true });
	renderer.setSize( window.innerWidth, window.innerHeight );
	renderer.physicallyCorrectLights = true;
	renderer.gammaFactor = 2.2;
	// TODO decide if we want shadows
	// renderer.shadowMap.enabled = true;
	// renderer.shadowMap.autoUpdate = true;
	return renderer;
}

export { createRenderer };