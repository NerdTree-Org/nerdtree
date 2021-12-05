import PaginatePosts from '~/api-wrapper/blog/query/paginate'
import ByAuthor from '~/api-wrapper/blog/query/by_author'
import ById from '~/api-wrapper/blog/query/by_id'

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
  update: {},
  query: {
    ById,
    ByAuthor,
    PaginatePosts,
  },
  voting: {},
};
