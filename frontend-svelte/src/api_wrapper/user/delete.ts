import { makeRequest, type RequestResult, type StatusPayload } from '../common/request';

interface DeleteUserPayload {
    user_id: string;
}

const DELETE_USER_ROUTE = 'user/delete/user';

export async function deleteUser(
    payload: DeleteUserPayload
): Promise<RequestResult<StatusPayload>> {
    return await makeRequest<DeleteUserPayload, StatusPayload>(payload, DELETE_USER_ROUTE, true);
}
