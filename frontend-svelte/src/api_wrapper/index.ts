import { loginRequest } from './auth/login';
import { registrationRequest } from './auth/register';
import { getCurrentUser, getUserByEmail, getUserById, getUserByUsername } from './user/query';
import { passwordResetRequest, passwordResetToken } from './auth/password_reset';
import { refreshAccessToken } from './auth/refresh_accesstoken';
import updateProfilePic, {
	updateEmail,
	updateFirstName,
	updateLastName,
	updatePassword
} from './user/update';
import { getDiscordToken } from './user/get';
import { deleteUser } from './user/delete';

export const API = {
	auth: {
		login: loginRequest,
		register: registrationRequest,
		refresh_token: refreshAccessToken,
		password_reset: {
			request: passwordResetRequest,
			token: passwordResetToken
		}
	},
	user: {
		update: {
			firstname: updateFirstName,
			lastname: updateLastName,
			email: updateEmail,
			password: updatePassword,
			profile_pic: updateProfilePic
		},
		query: {
			id: getUserById,
			username: getUserByUsername,
			email: getUserByEmail,
			current: getCurrentUser
		},
		get: {
			discord_token: getDiscordToken
		},
		delete: {
			user: deleteUser
		}
	}
};
