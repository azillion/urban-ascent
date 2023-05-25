import * as THREE from 'three';

class PathGroup {
	constructor(points = [new THREE.Vector2(0,0), new THREE.Vector2(1,1)], isCurved = false) {
		this.group = new THREE.Group();
		this.group.tick = (delta) => {};
		this.points = [];
		for (let i = 0; i < points.length; i++) {
			const point = new THREE.Vector3(points[i].x, 0, points[i].y).round()
			if (i == 0) this.startVector = point;
			if (i == points.length - 1) this.endVector = point;
			this.points.push(point);
		}

		// if line is curved, use CatmullRomCurve3
		// let geometry;
		if (isCurved) {
			const curve = new THREE.CatmullRomCurve3([this.startVector, this.endVector]);
			const points = curve.getPoints(50);
			const geometry = new THREE.BufferGeometry().setFromPoints(points);
			const material = new THREE.LineBasicMaterial({ color: 0xff0000 });
			this.path = new THREE.Line(geometry, material);
		} else {
			const geometry = new THREE.BufferGeometry().setFromPoints(this.points);
			const material = new THREE.LineBasicMaterial({ color: 0xff0000 });
			this.path = new THREE.Line(geometry, material);
		}
		// let shape = new THREE.Shape();
        // shape.moveTo(this.startVector.x, this.startVector.z);

        // for(let i=1; i < this.points.length; i++) {
        //     shape.lineTo(this.points[i].x, this.points[i].z);
        // }
		// const material = new THREE.MeshStandardMaterial({ color: 0xff0000 });
		// this.path = new THREE.ShapeGeometry(shape, material);
	}

	// function computeNodePoints() {
	// 	// 
	// }

	getPath() {
		return this.path;
	}

	getGroup() {
		return this.group;
	}

	getStartVector() {
		return this.startVector;
	}

	getEndVector() {
		return this.endVector;
	}
}

export { PathGroup };