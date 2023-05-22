import { configureStore } from '@reduxjs/toolkit'
import toolbarReducer from './toolbar'

export default configureStore({
  reducer: {
  toolbarReducer
  }
})