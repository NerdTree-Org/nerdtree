import { makeRequest, type RequestResult } from '../common/request';
import type { User } from '../../interfaces/user';

interface GetUserByIdPayload {
    id: string;
}

const GET_USER_BY_ID_ROUTE = 'user/query/id';

export async function getUserById(payload: GetUserByIdPayload): Promise<RequestResult<User>> {
    const result = await makeRequest<GetUserByIdPayload, User>(
        payload,
        GET_USER_BY_ID_ROUTE,
        false
    );

    if (result.success && result.value) {
        return result;
    } else {
        return {
            value: null,
            success: false,
            error: "Can't find user"
        };
    }
}

interface GetUserByUsernamePayload {
    username: string;
}

const GET_USER_BY_USERNAME_ROUTE = 'user/query/username';

export async function getUserByUsername(
    payload: GetUserByUsernamePayload
): Promise<RequestResult<User>> {
    return await makeRequest<GetUserByUsernamePayload, User>(
        payload,
        GET_USER_BY_USERNAME_ROUTE,
        false
    );
}

interface GetUserByEmailPayload {
    email: string;
}

const GET_USER_BY_EMAIL_ROUTE = 'user/query/email';

export async function getUserByEmail(payload: GetUserByEmailPayload): Promise<RequestResult<User>> {
    return await makeRequest<GetUserByEmailPayload, User>(payload, GET_USER_BY_EMAIL_ROUTE, false);
}

const GET_CURRENT_USER_ROUTE = 'user/query/current';

export async function getCurrentUser(): Promise<RequestResult<User>> {
    return await makeRequest<null, User>(null, GET_CURRENT_USER_ROUTE, true);
}
