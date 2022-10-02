import { User } from "../interfaces/user";
import { createSlice } from "@reduxjs/toolkit";
import { HYDRATE } from "next-redux-wrapper";
import { AppState } from "./store";

export interface AuthState {
  is_authenticated: boolean;
  last_accesstoken_generation_time: Date | null;
  user: User | null;
}

const initialState: AuthState = {
  is_authenticated: false,
  last_accesstoken_generation_time: null,
  user: null,
};

export const authSlice = createSlice({
  name: "auth",
  initialState,
  reducers: {
    setAuthState(state, action) {
      state.user = action.payload;
    },
  },
  extraReducers: {
    [HYDRATE]: (state, action) => {
      return {
        ...state,
        ...action.payload.user,
      };
    },
  },
});

export const { setAuthState } = authSlice.actions;
export const selectAuthUser = (state: AppState) => state.auth.user;
export const selectAuthStatus = (state: AppState) =>
  state.auth.is_authenticated;
export const selectLastAccessTokenGenerationTime = (state: AppState) =>
  state.auth.last_accesstoken_generation_time;
export default authSlice.reducer;
