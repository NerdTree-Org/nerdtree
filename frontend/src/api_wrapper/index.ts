import { loginRequest } from './auth/login';
import { registrationRequest } from './auth/register';
import { getCurrentUser, getUserByEmail, getUserById, getUserByUsername } from './user/query';
import { passwordResetRequest, passwordResetToken } from './auth/password_reset';
import { refreshAccessToken } from './auth/refresh_accesstoken';
import updateProfilePic, {
    updateEmail,
    updateFirstName,
    updateLastName,
    updatePassword
} from './user/update';
import { getDiscordToken } from './user/get';
import { deleteUser } from './user/delete';
import {
    deletePost,
    newPost,
    updateApproval,
    updatePostBody,
    updateThumbnail,
    updateTitle
} from './blog/update';
import {
    getPostById,
    getPostByIdIncludingUnapproved,
    getPostsByAuthorId,
    getPostsByAuthorIdIncludingUnapproved,
    paginatePosts,
    paginatePostsIncludingUnapproved
} from './blog/query';
import { addDownvote, addUpvote, getUserVoteForPost, getVotes } from './blog/vote';
import { deleteComment, editComment, newComment } from './comment/update';
import { getCommentsByPost, getCommentsByUser } from './comment/query';

export const API = {
    auth: {
        login: loginRequest,
        register: registrationRequest,
        refresh_token: refreshAccessToken,
        password_reset: {
            request: passwordResetRequest,
            token: passwordResetToken
        }
    },
    post: {
        update: {
            new: newPost,
            thumbnail: updateThumbnail,
            body: updatePostBody,
            title: updateTitle,
            update_approval: updateApproval,
            delete: deletePost
        },
        query: {
            unapproved: {
                paginate: paginatePostsIncludingUnapproved,
                id: getPostByIdIncludingUnapproved,
                author_id: getPostsByAuthorIdIncludingUnapproved
            },
            paginate: paginatePosts,
            id: getPostById,
            author_id: getPostsByAuthorId
        },
        vote: {
            upvote: addUpvote,
            downvote: addDownvote,
            votes: getVotes,
            by_current_user: getUserVoteForPost
        }
    },
    comment: {
        update: {
            new: newComment,
            edit: editComment,
            delete: deleteComment
        },
        query: {
            by_post: getCommentsByPost,
            by_user: getCommentsByUser
        }
    },
    user: {
        update: {
            firstname: updateFirstName,
            lastname: updateLastName,
            email: updateEmail,
            password: updatePassword,
            profile_pic: updateProfilePic
        },
        query: {
            id: getUserById,
            username: getUserByUsername,
            email: getUserByEmail,
            current: getCurrentUser
        },
        get: {
            discord_token: getDiscordToken
        },
        delete: {
            user: deleteUser
        }
    }
};
