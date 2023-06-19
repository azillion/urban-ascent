import * as THREE from 'three';
import { globalEventManager } from '../../../services/GlobalEventManager';

const NODE_EVENTS = Object.freeze({
	NODE_MOUSE_DOWN: 'nodeMouseDown',
	NODE_MOUSE_MOVE: 'nodeMouseMove',
	NODE_MOUSE_UP: 'nodeMouseUp',
});

function createRoadCreationNode(radius = 1, color = 0XBEE2FF) {
    const segments = 64;  // Number of segments in the circle (increase for higher resolution)
    const circleGeometry = new THREE.CircleGeometry(radius, segments);
    
    // Rotate the circle so it's horizontal
    circleGeometry.rotateX(-Math.PI / 2);

	color = new THREE.Color(color);

    const circleMaterial = new THREE.MeshStandardMaterial({ color: color, side: THREE.DoubleSide });
    
    const circle = new THREE.Mesh(circleGeometry, circleMaterial);
	circle.name = 'pathCreationNode';
	circle.position.y = 0.1;
	circle.visible = false;
	registerBrowserEvents(circle);
    return circle;
}

function registerBrowserEvents() {
	const handleEvent = (gameEvent) => (intersections, event) => {
		globalEventManager.dispatchEvent(gameEvent, intersections, event);
	};
	globalEventManager.registerEventListener('mousedown', handleEvent(NODE_EVENTS.NODE_MOUSE_DOWN));
	globalEventManager.registerEventListener('mouseup', handleEvent(NODE_EVENTS.NODE_MOUSE_UP));
	globalEventManager.registerEventListener('mousemove', handleEvent(NODE_EVENTS.NODE_MOUSE_MOVE));
}

export { createRoadCreationNode, NODE_EVENTS };