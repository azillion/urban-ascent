import * as THREE from 'three';

class DragManager {
	constructor(draggedEntity) {
		this.isDragging = false;
		this.draggedEntity = draggedEntity;
		this.previousMousePosition = new THREE.Vector2();
	}

	onMouseDown(event) {
		console.log(event)
		this.isDragging = true;
		this.previousMousePosition.set(event.clientX, event.clientY);
	}

	onMouseMove(event) {
		if (!this.isDragging) return;

		const mouseDelta = new THREE.Vector2();
		mouseDelta.subVectors(this.previousMousePosition, new THREE.Vector2(event.clientX, event.clientY));
		const rotationQuaternion = new THREE.Quaternion().setFromEuler(
			new THREE.Euler(
				-mouseDelta.y * 0.01,  // Rotation around x-axis
				-mouseDelta.x * 0.01,  // Rotation around y-axis
				0,                     // No rotation around z-axis
				'XYZ'
			)
		);

		this.draggedEntity.applyQuaternion(rotationQuaternion);
		this.previousMousePosition.set(event.clientX, event.clientY);
		this.draggedEntity.updateMatrix();
	}

	onMouseUp() {
		this.isDragging = false;
	}
}

export { DragManager };