import { createSlice } from '@reduxjs/toolkit'

export const namespaceSlice = createSlice({
  name: 'counter',
  initialState: {
    ns: [] ,
  },
  reducers: {
    update: (state) => {
        //TODO set to fetched value from tauri
        state.ns = ["hello"]
    }
  },
})

// Action creators are generated for each case reducer function
export const { update } = namespaceSlice.actions

export default namespaceSlice.reducer