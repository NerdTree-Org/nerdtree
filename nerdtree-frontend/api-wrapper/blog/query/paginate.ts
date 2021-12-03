import { StatusPayload } from '~/api-wrapper/common'
import { Post } from '~/api-wrapper/blog'

export type PaginatedPosts = {
  page: Post[],
  // eslint-disable-next-line camelcase
  current_page: Number,
  // eslint-disable-next-line camelcase
  max_page: Number,
}

export default async function PaginatePosts(
  page: Number,
  perPage: Number,
): Promise<StatusPayload<PaginatedPosts>>
{
  const payload = {
    page,
    per_page: perPage,
  }

  try {
    const req = await fetch(`${process.env.NERDTREE_API_URL}/post/query/paginate`, {
      method: 'post',
      body: JSON.stringify(payload),
      headers: {
        'content-type': 'application/json'
      },
    });
    const jsonBody = await req.json();

    if (req.status !== 200) {
      return {
        success: false,
        message: jsonBody.error,
      }
    }
    else {
      return {
        success: true,
        value: jsonBody
      };
    }
  }
  catch {
    return {
      success: false,
      message: "Cannot connect to server",
    }
  }
}
