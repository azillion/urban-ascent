import { configureStore } from '@reduxjs/toolkit';
import entities from './entities';

export default configureStore({
	reducer: entities,
	devTools: true,
});