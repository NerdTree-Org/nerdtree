import { makeRequest, type RequestResult, type StatusPayload } from '../common/request';
import { ENV } from '../../env';
import { getAccessToken } from '../common/store_auth_info_cookie';

interface UpdateFirstnamePayload {
    firstname: string;
}

const UPDATE_FIRSTNAME_ROUTE = 'user/update/firstname';

export async function updateFirstName(
    payload: UpdateFirstnamePayload
): Promise<RequestResult<StatusPayload>> {
    return await makeRequest<UpdateFirstnamePayload, StatusPayload>(
        payload,
        UPDATE_FIRSTNAME_ROUTE,
        true
    );
}

interface UpdateLastnamePayload {
    lastname: string;
}

const UPDATE_LASTNAME_ROUTE = 'user/update/lastname';

export async function updateLastName(
    payload: UpdateLastnamePayload
): Promise<RequestResult<StatusPayload>> {
    return await makeRequest<UpdateLastnamePayload, StatusPayload>(
        payload,
        UPDATE_LASTNAME_ROUTE,
        true
    );
}

interface UpdateEmailPayload {
    lastname: string;
}

const UPDATE_EMAIL_ROUTE = 'user/update/email';

export async function updateEmail(
    payload: UpdateEmailPayload
): Promise<RequestResult<StatusPayload>> {
    return await makeRequest<UpdateEmailPayload, StatusPayload>(payload, UPDATE_EMAIL_ROUTE, true);
}

interface UpdatePasswordPayload {
    old_password: string;
    new_password: string;
}

const UPDATE_PASSWORD_ROUTE = 'user/update/password';

export async function updatePassword(
    payload: UpdatePasswordPayload
): Promise<RequestResult<StatusPayload>> {
    return await makeRequest<UpdatePasswordPayload, StatusPayload>(
        payload,
        UPDATE_PASSWORD_ROUTE,
        true
    );
}

interface UpdateProfilePicPayload {
    form_data: FormData;
}

const UPDATE_PROFILE_PIC_ROUTE = 'user/update/profile_pic';

export default async function updateProfilePic(
    payload: UpdateProfilePicPayload
): Promise<StatusPayload> {
    let req;
    try {
        req = await fetch(`${ENV.api_address}/${UPDATE_PROFILE_PIC_ROUTE}`, {
            method: 'POST',
            body: payload.form_data,
            headers: {
                authorization: `bearer ${getAccessToken()}`
            }
        });
    } catch (e) {
        return {
            success: false,
            message: 'Failed to connect to the server'
        };
    }

    return {
        success: true,
        message: undefined
    };
}
