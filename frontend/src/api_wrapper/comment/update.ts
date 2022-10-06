import { makeRequest, type RequestResult, type StatusPayload } from '../common/request';

interface NewCommentPayload {
    post_id: string;
    body: string;
}

const NEW_COMMENT_ROUTE = 'comment/update/new';

export async function newComment(
    payload: NewCommentPayload
): Promise<RequestResult<StatusPayload>> {
    return await makeRequest<NewCommentPayload, StatusPayload>(payload, NEW_COMMENT_ROUTE, true);
}

interface EditCommentPayload {
    comment_id: string;
    body: string;
}

const EDIT_COMMENT_ROUTE = 'comment/update/edit';

export async function editComment(
    payload: EditCommentPayload
): Promise<RequestResult<StatusPayload>> {
    return await makeRequest<EditCommentPayload, StatusPayload>(payload, EDIT_COMMENT_ROUTE, true);
}

interface DeleteCommentPayload {
    comment_id: string;
}

const DELETE_COMMENT_ROUTE = 'comment/update/delete';

export async function deleteComment(
    payload: DeleteCommentPayload
): Promise<RequestResult<StatusPayload>> {
    return await makeRequest<DeleteCommentPayload, StatusPayload>(
        payload,
        DELETE_COMMENT_ROUTE,
        true
    );
}
