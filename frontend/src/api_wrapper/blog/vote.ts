import { makeRequest, type RequestResult, type StatusPayload } from '../common/request';

interface AddUpvotePayload {
    post_id: string;
}

const ADD_UPVOTE_ROUTE = 'post/vote/upvote';

export async function addUpvote(payload: AddUpvotePayload): Promise<RequestResult<StatusPayload>> {
    return await makeRequest<AddUpvotePayload, StatusPayload>(payload, ADD_UPVOTE_ROUTE, true);
}

interface AddDownvotePayload {
    post_id: string;
}

const ADD_DOWNVOTE_ROUTE = 'post/vote/downvote';

export async function addDownvote(
    payload: AddDownvotePayload
): Promise<RequestResult<StatusPayload>> {
    return await makeRequest<AddDownvotePayload, StatusPayload>(payload, ADD_DOWNVOTE_ROUTE, true);
}

interface GetVotesPayload {
    post_id: string;
}

interface GetVotesReturnPayload {
    votes: number;
}

const GET_VOTES_ROUTE = 'post/vote/votes';

export async function getVotes(
    payload: GetVotesPayload
): Promise<RequestResult<GetVotesReturnPayload>> {
    return await makeRequest<GetVotesPayload, GetVotesReturnPayload>(
        payload,
        GET_VOTES_ROUTE,
        false
    );
}

interface GetUserVoteForPostPayload {
    post_id: string;
}

interface GetUserVoteForPostReturnPayload {
    vote: number;
}

const GET_USER_VOTES_FOR_POST_ROUTE = 'post/vote/by_current_user';

export async function getUserVoteForPost(
    payload: GetUserVoteForPostPayload
): Promise<RequestResult<GetUserVoteForPostReturnPayload>> {
    return await makeRequest<GetUserVoteForPostPayload, GetUserVoteForPostReturnPayload>(
        payload,
        GET_USER_VOTES_FOR_POST_ROUTE,
        true
    );
}
