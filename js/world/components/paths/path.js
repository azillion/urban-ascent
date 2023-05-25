/**
 * @class Path
 * @description A path is a collection of points that can be used to create a route
 * @example const path = new Path();
 * path.addPoint(new THREE.Vector3(0, 0, 0));
 * path.addPoint(new THREE.Vector3(1, 0, 0));
 * path.addPoint(new THREE.Vector3(2, 0, 0));
 * path.points; // [THREE.Vector3(0, 0, 0), THREE.Vector3(1, 0, 0), THREE.Vector3(2, 0, 0)]
 * path.getPointByIndex(0); // THREE.Vector3(0, 0, 0)
 * path.getPointByPosition(new THREE.Vector3(1, 0, 0)); // THREE.Vector3(1, 0, 0)
 * path.getPointPosition(new THREE.Vector3(1, 0, 0)); // 1
 * path.removePoint(new THREE.Vector3(1, 0, 0));
 * path.points; // [THREE.Vector3(0, 0, 0), THREE.Vector3(2, 0, 0)]
 * path.length; // 2
 */
class Path {
	// TODO: Should we use a linked list instead of an array?
	constructor() {
		this._points = [];
		this._pointsHash = {};
		this._segments = [];
		this._segmentsHash = {};
	}

	addPoint(point) {
		this._points.push(point);
		this._pointsHash[point] = point;
	}

	removePoint(point) {
		this._points.splice(this._points.indexOf(point), 1);
		delete this._pointsHash[point];
	}

	/**
	 * @returns {THREE.Vector3[]}
	 * @description Returns the points of the path
	 */
	get points() {
		return this._points;
	}

	/**
	 * @param {THREE.Vector3[]} points
	 * @description Sets the points of the path
	 * @example path.points = [new THREE.Vector3(0, 0, 0), new THREE.Vector3(1, 0, 0)];
	 * path.points; // [THREE.Vector3(0, 0, 0), THREE.Vector3(1, 0, 0)]
	 */
	set points(points) {
		this._points = points;
		this._pointsHash = {};
		for (let i = 0; i < points.length; i++) {
			const point = points[i];
			this._pointsHash[point] = point;
		}
	}

	get length() {
		return this._points.length;
	}

	getPointByIndex(index) {
		return this._points[index];
	}

	/**
	 * @param {THREE.Vector3} position
	 * @returns {THREE.Vector3}
	 * @description Returns the point at the given position
	 * @example path.getPointByPosition(new THREE.Vector3(0, 0, 0));
	 */
	getPointByPosition(position) {
		return this._pointsHash[position];
	}

	getPointPosition(point) {
		return this._points.indexOf(point);
	}
}

class BezierSegment {
	constructor(start, end, control1, control2) {
		this.start = start;
		this.end = end;
		this.control1 = control1;
		this.control2 = control2;
	}
}