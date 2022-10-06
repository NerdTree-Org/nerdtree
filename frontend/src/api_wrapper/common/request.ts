// For routes that only returns status with optional message
import { ENV } from '../../env';
import { getAccessToken } from './store_auth_info_cookie';

export interface StatusPayload {
    success: boolean;
    message?: string;
}

export interface RequestResult<T> {
    value: T | null;
    success: boolean;
    error: string | null;
}

export async function makeRequest<P, R>(
    payload: P,
    route: string,
    requiresAuth: boolean
): Promise<RequestResult<R>> {
    const requestHeaders: HeadersInit = new Headers();
    requestHeaders.set('Content-Type', 'application/json');
    if (requiresAuth) {
        requestHeaders.set('Authorization', `bearer ${getAccessToken()}`);
    }

    let req;
    try {
        req = await fetch(`${ENV.api_address}/${route}`, {
            method: 'POST',
            headers: requestHeaders,
            body: JSON.stringify(payload)
        });
    } catch (e) {
        return {
            value: null,
            success: false,
            error: JSON.stringify(e)
        };
    }

    const jsonBody = await req.json();
    if (req.status != 200) {
        return {
            value: null,
            success: false,
            error: jsonBody.error
        };
    } else {
        return {
            value: jsonBody as R,
            success: true,
            error: null
        };
    }
}
