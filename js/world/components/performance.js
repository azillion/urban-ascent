import * as THREE from 'three';
import { createCube } from './cube';

function createBunchOfShit() {
	const group = new THREE.Group();
	const cubes = [];

	group.tick = (delta) => {
		for (const cube of cubes) {
			cube.tick(delta);
		}
	};

	for (let i = 0; i < 10000; i++) {
		const plane = createPlane();
		plane.position.x = (Math.random() - 0.5) * 1000;
		plane.position.z = (Math.random() - 0.5) * 1000;
		const cube1 = createCube();
		cube1.position.x = 4;
		cubes.push(cube1);
		plane.add(cube1);
		const cube2 = createCube();
		cube2.position.x = -4;
		cube2.material.color.set('blue');
		cubes.push(cube2);
		plane.add(cube2);
		group.add(plane);
	}

	return group;
}

const xAngle = THREE.MathUtils.degToRad(-90);
const zAngle = THREE.MathUtils.degToRad(90);

function createPlane() {
	const randomWidth = Math.random() * 100;
	const randomHeight = Math.random() * 100;
	const geometry = new THREE.PlaneGeometry(randomWidth, randomHeight, 10, 10);
	const material = new THREE.MeshStandardMaterial( { color: 'darkslategray' } );
	const plane = new THREE.Mesh( geometry, material );
	plane.position.y = 0.1001;
	plane.rotateX(xAngle);
	plane.rotateZ(zAngle);
	return plane;
}

export { createBunchOfShit };