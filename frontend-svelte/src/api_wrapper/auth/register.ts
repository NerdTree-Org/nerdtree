import { makeRequest, type RequestResult, type StatusPayload } from '../common/request';

export interface RegisterPayload {
    username: string;
    firstname: string;
    lastname: string;
    email: string;
    password: string;
    facebook_id: string;
}

const REGISTRATION_ROUTE = 'auth/register';

export async function registrationRequest(
    payload: RegisterPayload
): Promise<RequestResult<StatusPayload>> {
    return await makeRequest<RegisterPayload, StatusPayload>(payload, REGISTRATION_ROUTE, false);
}
