import PaginatePosts from '~/api-wrapper/blog/query/paginate'

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
    PaginatePosts,
  },
  voting: {},
};
