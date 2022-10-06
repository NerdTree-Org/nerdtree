import { makeRequest, type RequestResult } from '../common/request';
import { setCookie } from 'typescript-cookie';

interface RefreshAccessTokenPayload {
    refresh_token: string;
}

interface RefreshAccessTokenReturnPayload {
    access_token: string;
}

const REFRESH_ACCESSTOKEN_ROUTE = 'auth/refresh_token';

export async function refreshAccessToken(
    payload: RefreshAccessTokenPayload
): Promise<RequestResult<RefreshAccessTokenReturnPayload>> {
    const result = await makeRequest<RefreshAccessTokenPayload, RefreshAccessTokenReturnPayload>(
        payload,
        REFRESH_ACCESSTOKEN_ROUTE,
        false
    );

    if (result.success && result.value) {
        setCookie('accesstoken', result.value.access_token);
    }

    return result;
}
