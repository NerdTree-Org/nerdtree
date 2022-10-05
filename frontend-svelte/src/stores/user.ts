import { writable } from 'svelte/store';
import type { User } from '../interfaces/user';

interface AuthInfo {
    user: User;
    last_login: Date;
    access_token: string;
    refresh_token: string;
}

interface AuthState {
    info: AuthInfo | null;
}

export const AuthenticationStatus = writable<AuthState>({ info: null });
