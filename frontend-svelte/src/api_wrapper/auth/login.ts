import { makeRequest, type RequestResult, type StatusPayload } from '../common/request';
import { AuthenticationStatus } from '../../stores/user';
import { getUserIdFromAccessToken } from '../common/jwt';
import { getUserById } from '../user/query';
import { storeAuthInfoCookie } from '../common/store_auth_info_cookie';

interface LoginRequestPayload {
    username: string;
    password: string;
}

interface TokensPaylaod {
    refreshtoken: string;
    accesstoken: string;
}

const LOGIN_ROUTE = 'auth/login';

export async function loginRequest(payload: LoginRequestPayload): Promise<RequestResult<null>> {
    const result = await makeRequest<LoginRequestPayload, TokensPaylaod>(
        payload,
        LOGIN_ROUTE,
        false
    );

    if (result.success && result.value) {
        const { accesstoken, refreshtoken } = result.value;
        const userId = getUserIdFromAccessToken(accesstoken);
        const result1 = await getUserById({ id: userId });
        const loginTime = new Date();
        let user;
        if (result1.success && result1.value) {
            user = result1.value;
        } else {
            return {
                value: null,
                success: false,
                error: 'Invalid credentials'
            };
        }

        storeAuthInfoCookie(accesstoken, refreshtoken, loginTime);
        AuthenticationStatus.set({
            info: {
                access_token: accesstoken,
                refresh_token: refreshtoken,
                last_login: loginTime,
                user
            }
        });

        return {
            error: '',
            value: null,
            success: true
        };
    } else {
        return {
            value: null,
            success: false,
            error: 'Invalid credentials'
        };
    }
}
