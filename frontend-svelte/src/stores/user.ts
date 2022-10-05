import { writable } from 'svelte/store';

export interface User {
	id: string;
	username: string;
	firstname: string;
	lastname: string;
	email: string;
	profile_pic: string | null;
	is_admin: boolean;
	facebook_id: string;
	discord_token: string;
	has_used_discord_token: boolean;
}

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
