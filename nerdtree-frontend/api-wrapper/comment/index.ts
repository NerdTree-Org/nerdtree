import New from '~/api-wrapper/comment/update/new'
import Edit from '~/api-wrapper/comment/update/edit'
import Delete from '~/api-wrapper/comment/update/delete'
import ByPost from '~/api-wrapper/comment/query/by_post'
import ByUser from '~/api-wrapper/comment/query/by_user'

export type Comment = {
  id: String,
  // eslint-disable-next-line camelcase
  post_id: String,
  // eslint-disable-next-line camelcase
  author_id: String | null,
  body: String,
}

export type PaginatedComments = {
  // eslint-disable-next-line camelcase
  current_page: Number,
  // eslint-disable-next-line camelcase
  max_page: Number,
  page: Comment[],
}

export default {
  update: {
    New,
    Edit,
    Delete,
  },
  query: {
    ByPost,
    ByUser,
  }
}
