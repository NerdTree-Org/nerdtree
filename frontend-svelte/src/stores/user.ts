import {writable} from 'svelte/store';
import type {User} from "../interfaces/user";

interface AuthInfo {
    user: User;
    last_login: Date;
    access_token: String;
    refresh_token: String;
}

interface AuthState {
    info: AuthInfo | null;
}

export const AuthenticationStatus = writable<AuthState>({ info: null });
