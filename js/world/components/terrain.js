import * as THREE from 'three';
import { globalEventManager } from '../../services/GlobalEventManager';
// import { ImprovedNoise } from './noise';

let terrain;

const TERRAIN_EVENTS = {
	TERRAIN_CLICK: 'terrainClick',
	TERRAIN_MOUSE_DOWN: 'terrainMouseDown',
	TERRAIN_MOUSE_UP: 'terrainMouseUp',
	TERRAIN_MOUSE_MOVE: 'terrainMouseMove',
};

function createTerrain(width, height) {
	// const data = generateHeight(worldWidth, worldDepth);
	// const geometry = new THREE.PlaneGeometry(7500, 7500, worldWidth - 1, worldDepth - 1);
	const geometry = new THREE.PlaneGeometry(width, height, 1, 1);
	geometry.rotateX( - Math.PI / 2 );

	// const vertices = geometry.attributes.position.array;

	// for ( let i = 0, j = 0, l = vertices.length; i < l; i ++, j += 3 ) {
	// 	vertices[ j + 1 ] = data[ i ] * 10;
	// }
	
	// const texture = THREE.CanvasTexture(generateTexture(data, worldWidth, worldDepth));
	// texture.wrapS = THREE.ClampToEdgeWrapping;
	// texture.wrapT = THREE.ClampToEdgeWrapping;
	// texture.colorSpace = THREE.SRGBColorSpace;
	const material = new THREE.MeshStandardMaterial({
		color: '#5b9877',
		// wireframe: true,
	});

	terrain = new THREE.Mesh(geometry, material);
	terrain.position.y = 0;

	// geometry.computeBoundingBox();
	// let zMax = geometry.boundingBox.max.z;
	// let xMax = geometry.boundingBox.max.x;
	// console.log(geometry.boundingBox)
	// terrain.position.set(-xMax / 2, 0, -zMax / 2);
	// console.log(terrain.position)
	// terrain.translateX(-xMax / 2);
	// terrain.translateZ(-zMax / 2);
	// terrain.position.y = -256;

	// terrain.rotation.x = Math.PI * -0.5;
	// terrain.position.set(0,0,0);
	terrain.receiveShadow = true;

	registerBrowserEvents();
	terrain.name = 'terrain';

	return terrain;
}

function registerBrowserEvents() {
	const handleEvent = (gameEvent) => (intersections, event) => {
		if (intersections.filter((intersection) => intersection.object.name === 'terrain').length > 0) {
			globalEventManager.dispatchEvent(gameEvent, intersections, event);
		}
	};
	globalEventManager.registerEventListener('mousedown', handleEvent(TERRAIN_EVENTS.TERRAIN_MOUSE_DOWN));
	globalEventManager.registerEventListener('mouseup', handleEvent(TERRAIN_EVENTS.TERRAIN_MOUSE_UP));
	globalEventManager.registerEventListener('mousemove', handleEvent(TERRAIN_EVENTS.TERRAIN_MOUSE_MOVE));
	globalEventManager.registerEventListener('click', handleEvent(TERRAIN_EVENTS.TERRAIN_CLICK));
}

// function handleEvent(gameEvent, ) {
// 	if (terrain && intersections.length > 0 && intersections[0].object.id === terrain.id) {
// 		globalEventManager.dispatchEvent(gameEvent, intersections);
// 	}
// }

// function handleMouseDown(intersections) {
// 	if (terrain && intersections.length > 0 && intersections[0].object.id === terrain.id) {
// 		globalEventManager.dispatchEvent(EVENTS.TERRAIN_MOUSE_DOWN, intersections);
// 	}
// }

// function handleMouseUp(intersections) {
// 	if (terrain && intersections.length > 0 && intersections[0].object.id === terrain.id) {
// 		globalEventManager.dispatchEvent(EVENTS.TERRAIN_MOUSE_UP, intersections);
// 	}
// }

// function handleMouseMove(intersections) {
// 	if (terrain && intersections.length > 0 && intersections[0].object.id === terrain.id) {
// 		globalEventManager.dispatchEvent(EVENTS.TERRAIN_MOUSE_MOVE, intersections);
// 	}
// }

// function handleClick(intersections) {
// 	if (terrain && intersections.length > 0 && intersections[0].object.id === terrain.id) {
// 		globalEventManager.dispatchEvent(EVENTS.TERRAIN_CLICK, intersections);
// 	}
// }


// function generateHeight( width, height ) {
// 	let seed = Math.PI / 4;
// 	window.Math.random = function () {
// 		const x = Math.sin( seed ++ ) * 10000;
// 		return x - Math.floor( x );
// 	};

// 	const size = width * height, data = new Uint8Array( size );
// 	const perlin = new ImprovedNoise(), z = Math.random() * 100;

// 	let quality = 1;

// 	for ( let j = 0; j < 4; j ++ ) {

// 		for ( let i = 0; i < size; i ++ ) {

// 			const x = i % width, y = ~ ~ ( i / width );
// 			data[ i ] += Math.abs( perlin.noise( x / quality, y / quality, z ) * quality * 1.75 );

// 		}

// 		quality *= 5;

// 	}

// 	return data;
// }

// function generateTexture(data, width, height) {
// 	let context, image, imageData, shade;

// 	const vector3 = new THREE.Vector3( 0, 0, 0 );

// 	const sun = new THREE.Vector3( 1, 1, 1 );
// 	sun.normalize();

// 	const canvas = document.createElement( 'canvas' );
// 	canvas.width = width;
// 	canvas.height = height;

// 	context = canvas.getContext( '2d' );
// 	context.fillStyle = '#000';
// 	context.fillRect( 0, 0, width, height );

// 	image = context.getImageData( 0, 0, canvas.width, canvas.height );
// 	imageData = image.data;

// 	for ( let i = 0, j = 0, l = imageData.length; i < l; i += 4, j ++ ) {

// 		vector3.x = data[ j - 2 ] - data[ j + 2 ];
// 		vector3.y = 2;
// 		vector3.z = data[ j - width * 2 ] - data[ j + width * 2 ];
// 		vector3.normalize();

// 		shade = vector3.dot( sun );

// 		imageData[ i ] = ( 96 + shade * 128 ) * ( 0.5 + data[ j ] * 0.007 );
// 		imageData[ i + 1 ] = ( 32 + shade * 96 ) * ( 0.5 + data[ j ] * 0.007 );
// 		imageData[ i + 2 ] = ( shade * 96 ) * ( 0.5 + data[ j ] * 0.007 );

// 	}

// 	context.putImageData( image, 0, 0 );

// 	// Scaled 4x

// 	const canvasScaled = document.createElement( 'canvas' );
// 	canvasScaled.width = width * 4;
// 	canvasScaled.height = height * 4;

// 	context = canvasScaled.getContext( '2d' );
// 	context.scale( 4, 4 );
// 	context.drawImage( canvas, 0, 0 );

// 	image = context.getImageData( 0, 0, canvasScaled.width, canvasScaled.height );
// 	imageData = image.data;

// 	for ( let i = 0, l = imageData.length; i < l; i += 4 ) {

// 		const v = ~ ~ ( Math.random() * 5 );

// 		imageData[ i ] += v;
// 		imageData[ i + 1 ] += v;
// 		imageData[ i + 2 ] += v;

// 	}

// 	context.putImageData( image, 0, 0 );

// 	return canvasScaled;

// }

export { createTerrain, TERRAIN_EVENTS };