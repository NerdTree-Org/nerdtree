import PaginatePosts from '~/api-wrapper/blog/query/paginate'
import ByAuthor from '~/api-wrapper/blog/query/by_author'
import ById from '~/api-wrapper/blog/query/by_id'
import CreateNewPost from '~/api-wrapper/blog/update/new_post'
import DeletePost from '~/api-wrapper/blog/update/delete_post'
import ChangeApproval from '~/api-wrapper/blog/update/change_approval'
import Title from '~/api-wrapper/blog/update/title'
import Body from '~/api-wrapper/blog/update/body'
import Thumbnail from '~/api-wrapper/blog/update/thumbnail'
import Upvote from '~/api-wrapper/blog/voting/upvote'
import Downvote from '~/api-wrapper/blog/voting/downvote'
import GetUserVoteForPost from '~/api-wrapper/blog/voting/get_user_vote'
import GetVotes from '~/api-wrapper/blog/voting/get_votes'

export type Post = {
  id: String,
  isApproved: boolean,
  title: String,
  thumbnail: String | null,
  body: String,
  creationDate: Date,
  approvalDate: Date,
  postAuthor: Date,
};

export default {
  update: {
    CreateNewPost,
    DeletePost,
    ChangeApproval,
    Title,
    Body,
    Thumbnail,
  },
  query: {
    ById,
    ByAuthor,
    PaginatePosts,
  },
  voting: {
    Upvote,
    Downvote,
    GetUserVoteForPost,
    GetVotes,
  },
};
