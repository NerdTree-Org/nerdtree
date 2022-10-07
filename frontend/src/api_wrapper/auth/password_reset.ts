import { makeRequest, type RequestResult, type StatusPayload } from '../common/request';

interface PasswordResetRequestPayload {
    email: string;
}

const PASSWORD_RESET_ROUTE = 'auth/password_reset/request';

export async function passwordResetRequest(
    payload: PasswordResetRequestPayload
): Promise<RequestResult<StatusPayload>> {
    return await makeRequest<PasswordResetRequestPayload, StatusPayload>(
        payload,
        PASSWORD_RESET_ROUTE,
        false
    );
}

interface PasswordResetTokenPayload {
    user_id: string;
    reset_token: string;
    password: string;
}

const PASSWORD_RESET_TOKEN_ROUTE = 'auth/password_reset/token';

export async function passwordResetToken(
    payload: PasswordResetTokenPayload
): Promise<RequestResult<StatusPayload>> {
    return await makeRequest<PasswordResetTokenPayload, StatusPayload>(
        payload,
        PASSWORD_RESET_TOKEN_ROUTE,
        false
    );
}
