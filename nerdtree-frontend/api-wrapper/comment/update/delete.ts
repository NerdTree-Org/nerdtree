import NerdTreeSession from '~/store/session'
import { StatusPayload } from '~/api-wrapper/common'
import { Comment } from '~/api-wrapper/comment'

export default async function Delete(
  commentId: String,
  ctx: NerdTreeSession
): Promise<StatusPayload<Comment>> {
  const payload = {
    comment_id: commentId,
  }

  try {
    const req = await fetch(
      `${process.env.NERDTREE_API_URL}/comment/update/delete`,
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
