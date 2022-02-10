import { StatusPayload } from '~/api-wrapper/common'
import { Post } from '~/api-wrapper/blog'
import NerdTreeSession from '~/store/session'

export default async function ChangeApproval(
  postId: String,
  approvalState: boolean,
  ctx: NerdTreeSession
): Promise<StatusPayload<Post>> {
  const payload = {
    post_id: postId,
    approval_state: approvalState,
  }

  try {
    const req = await fetch(
      `${process.env.NERDTREE_API_URL}/post/update/update_approval`,
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
