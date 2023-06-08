import Dexie from 'dexie';

import packageJson from '../../package.json';

class GlobalSaveManager {
	constructor() {
		this._setupDatabase();
	}

	_setupDatabase() {
		try {
			const version = packageJson.version || '0.0.0';
			console.log('GlobalSaveManager version: ' + version);
			this.db = new Dexie('UrbanAscent-' + version);
			this.db.version(1).stores({
				games: 'save_title,data'
			});
		} catch (error) {
			console.error(error);
		}
	}

	clearDatabase() {
		try {
			this.db.delete();
			this._setupDatabase();
			return true;
		} catch (error) {
			console.error(error);
			return false;
		}
	}

	async getSavedGameTitles() {
		try {
			return await this.db.games.toCollection().primaryKeys() || [];
		} catch (error) {
			console.error(error);
			return [];
		}
	}

	async doesGameExist(gameSaveTitle) {
		try {
			const count = await this.db.games.get(gameSaveTitle).count();
			return count > 0;
		} catch (error) {
			console.error(error);
			return false;
		}
	}

	async saveGame(gameSaveTitle, gameSaveData) {
		try {
			if (await this.doesGameExist(gameSaveTitle)) {
				await this.db.games.update(gameSaveTitle, { data: gameSaveData });
				return;
			} 
			await this.db.games.add({ save_title: gameSaveTitle, data: gameSaveData });
		} catch (error) {
			console.error(error);
		}
	}

	async retrieveGameData(gameSaveTitle) {
		try {
			await this.db.games.get(gameSaveTitle).then(gameSave => {
				if (!gameSave) {
					return null;
				}
				return gameSave.data;
			});
		} catch (error) {
			console.error(error);
			return null;
		}
	}
}

const globalSaveManager = new GlobalSaveManager();

export default globalSaveManager;