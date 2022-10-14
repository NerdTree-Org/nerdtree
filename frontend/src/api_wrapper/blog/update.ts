import { makeRequest, type RequestResult, type StatusPayload } from '../common/request';
import type { Post } from '../../interfaces/post';
import { ENV } from '../../env';
import { getAccessToken } from '../common/store_auth_info_cookie';

interface NewPostPayload {
    title: string;
    body: string;
}

const NEW_POST_ROUTE = 'post/update/new';

export async function newPost(payload: NewPostPayload): Promise<RequestResult<Post>> {
    return await makeRequest<NewPostPayload, Post>(payload, NEW_POST_ROUTE, true);
}

interface UpdateThumbnailPayload {
    form_data: FormData;
}

const UPDATE_THUMBNAIL_ROUTE = 'post/update/thumbnail';

export async function updateThumbnail(
    payload: UpdateThumbnailPayload
): Promise<RequestResult<Post>> {
    let req;
    try {
        req = await fetch(`${ENV.api_address}/${UPDATE_THUMBNAIL_ROUTE}`, {
            method: 'POST',
            body: payload.form_data,
            headers: {
                authorization: `bearer ${getAccessToken()}`
            }
        });
    } catch (e) {
        return {
            success: false,
            value: null,
            error: 'Failed to connect to server'
        };
    }

    return {
        success: true,
        value: await req.json(),
        error: ''
    };
}

interface UpdatePostBodyPayload {
    post_id: string;
    body: string;
}

const UPDATE_POST_BODY_ROUTE = 'post/update/body';

export async function updatePostBody(payload: UpdatePostBodyPayload): Promise<RequestResult<Post>> {
    return await makeRequest<UpdatePostBodyPayload, Post>(payload, UPDATE_POST_BODY_ROUTE, true);
}

interface UpdateApprovalPayload {
    post_id: string;
    approval_state: boolean;
}

const UPDATE_APPROVAL_ROUTE = 'post/update/update_approval';

export async function updateApproval(payload: UpdateApprovalPayload): Promise<RequestResult<Post>> {
    return await makeRequest<UpdateApprovalPayload, Post>(payload, UPDATE_APPROVAL_ROUTE, true);
}

interface DeletePostPayload {
    post_id: string;
}

const DELETE_POST_ROUTE = 'post/update/delete';

export async function deletePost(
    payload: DeletePostPayload
): Promise<RequestResult<StatusPayload>> {
    return await makeRequest<DeletePostPayload, StatusPayload>(payload, DELETE_POST_ROUTE, true);
}

interface UpdateTitlePayload {
    post_id: string;
    new_title: string;
}

const UPDATE_TITLE_ROUTE = 'post/update/title';

export async function updateTitle(payload: UpdateTitlePayload): Promise<RequestResult<Post>> {
    return await makeRequest<UpdateTitlePayload, Post>(payload, UPDATE_TITLE_ROUTE, true);
}
