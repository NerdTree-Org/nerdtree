import type { Post } from '../../interfaces/post';
import { makeRequest, type RequestResult } from '../common/request';

interface PaginatePostsPayload {
    page: number;
    per_page: number;
}

interface PaginatePostsReturnPayload {
    current_page: number;
    max_page: number;
    page: Post[];
}

const PAGINATE_POSTS_ROUTE = 'post/query/paginate';

export async function paginatePosts(
    payload: PaginatePostsPayload
): Promise<RequestResult<PaginatePostsReturnPayload>> {
    return await makeRequest<PaginatePostsPayload, PaginatePostsReturnPayload>(
        payload,
        PAGINATE_POSTS_ROUTE,
        false
    );
}

interface GetPostByIdPayload {
    post_id: string;
}

const GET_POST_BY_ID_ROUTE = 'post/query/id';

export async function getPostById(payload: GetPostByIdPayload): Promise<RequestResult<Post>> {
    return await makeRequest<GetPostByIdPayload, Post>(payload, GET_POST_BY_ID_ROUTE, false);
}

interface GetPostsByAuthorId {
    author_id: string;
}

const GET_POSTS_BY_AUTHOR_ID_ROUTE = 'post/query/author_id';

export async function getPostsByAuthorId(
    payload: GetPostsByAuthorId
): Promise<RequestResult<Post[]>> {
    return await makeRequest<GetPostsByAuthorId, Post[]>(
        payload,
        GET_POSTS_BY_AUTHOR_ID_ROUTE,
        false
    );
}
