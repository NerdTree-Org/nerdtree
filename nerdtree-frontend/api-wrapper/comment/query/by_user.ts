import NerdTreeSession from '~/store/session'
import { StatusPayload } from '~/api-wrapper/common'
import { PaginatedComments } from '~/api-wrapper/comment'

export default async function ByUser(
  authorId: String,
  page: Number,
  perPage: Number,
  ctx: NerdTreeSession
): Promise<StatusPayload<PaginatedComments>> {
  const payload = {
    author_id: authorId,
    page,
    per_page: perPage,
  }

  try {
    const req = await fetch(
      `${process.env.NERDTREE_API_URL}/comment/query/by_user`,
      {
        method: 'post',
        body: JSON.stringify(payload),
        headers: {
          'content-type': 'application/json',
          authorization: `bearer ${ctx.AccessToken}`,
        },
      }
    )

    const jsonBody = await req.json()

    if (req.status !== 200) {
      return {
        success: false,
        message: jsonBody.error,
      }
    } else {
      return {
        success: true,
        value: jsonBody,
      }
    }
  } catch {
    return {
      success: false,
      message: 'Cannot connect to server',
    }
  }
}
