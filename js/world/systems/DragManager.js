import * as THREE from 'three';

class DragManager {
	constructor(draggedEntity) {
		this.isDragging = false;
		this.draggedEntity = draggedEntity;
		this.previousMousePosition = new THREE.Vector2();
	}

	onMouseDown() {
		this.isDragging = true;
	}

	onMouseMove(intersections) {
		if (!this.isDragging) return;
		let terrain;
		for (const intersection of intersections) {
			if (intersection.object.name === 'terrain') {
				terrain = intersection;
			}
		}
		const pos = terrain.point;
		pos.y = this.draggedEntity.position.y;
		this.draggedEntity.position.copy(pos);
	}

	onMouseUp() {
		this.isDragging = false;
	}
}

export { DragManager };