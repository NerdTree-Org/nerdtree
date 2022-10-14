import { makeRequest, type RequestResult } from '../common/request';
import type { Comment } from '../../interfaces/comment';

interface PaginatedCommentsReturnPayload {
    current_page: number;
    max_page: number;
    page: Comment[];
}

interface GetCommentsByPostPayload {
    post_id: string;
    page: number;
    per_page: number;
}

const GET_COMMENTS_BY_POST_ROUTE = 'comment/query/by_post';

export async function getCommentsByPost(
    payload: GetCommentsByPostPayload
): Promise<RequestResult<PaginatedCommentsReturnPayload>> {
    return await makeRequest<GetCommentsByPostPayload, PaginatedCommentsReturnPayload>(
        payload,
        GET_COMMENTS_BY_POST_ROUTE,
        false
    );
}

interface GetCommentsByUserPayload {
    author_id: string;
    page: number;
    per_page: number;
}

const GET_COMMENTS_BY_USER_ROUTE = 'comment/query/by_user';

export async function getCommentsByUser(
    payload: GetCommentsByUserPayload
): Promise<RequestResult<PaginatedCommentsReturnPayload>> {
    return await makeRequest<GetCommentsByUserPayload, PaginatedCommentsReturnPayload>(
        payload,
        GET_COMMENTS_BY_USER_ROUTE,
        true
    );
}
