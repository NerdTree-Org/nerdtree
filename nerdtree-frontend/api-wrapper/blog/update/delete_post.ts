import NerdTreeSession from '~/store/session'
import { StatusPayload } from '~/api-wrapper/common'

export default async function DeletePost(
  postId: String,
  ctx: NerdTreeSession
): Promise<StatusPayload> {
  const payload = {
    post_id: postId
  }

  try {
    const req = await fetch(`${process.env.NERDTREE_API_URL}/post/update/delete`, {
      method: 'post',
      body: JSON.stringify(payload),
      headers: {
        'content-type': 'application/json',
        'authorization': `bearer ${ctx.accessToken}`
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
      }
    }
  }
  catch {
    return {
      success: false,
      message: "Cannot connect to server",
    }
  }
}
