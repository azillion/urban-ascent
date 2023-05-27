let _listeners = {};

class GlobalEventManager {
	constructor() {
		_listeners = {};
	}

	registerEventListener(event, listener) {
		if (!_listeners[event]) {
			_listeners[event] = [];
		}
		_listeners[event].push(listener);
	}

	unregisterEventListener(event, listener) {
		if (!_listeners[event]) {
			return;
		}
		_listeners[event].splice(_listeners[event].indexOf(listener), 1);
	}

	dispatchEvent(eventString, data, event) {
		if (!_listeners[eventString]) {
			return;
		}
		for (let i = 0; i < _listeners[eventString].length; i++) {
			const listener = _listeners[eventString][i];
			listener(data, event);
		}
	}

	get listeners() {
		return _listeners;
	}

	set listeners(listeners) {
		_listeners = listeners;
	}
}

const globalEventManager = new GlobalEventManager();
export { globalEventManager };