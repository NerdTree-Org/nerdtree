import { StatusPayload } from '~/api-wrapper/common'
import { Post } from '~/api-wrapper/blog'
import NerdTreeSession from '~/store/session'

export default async function CreateNewPost(
  title: String,
  body: String,
  ctx: NerdTreeSession
): Promise<StatusPayload<Post>> {
  const payload = {
    title,
    body,
  }

  try {
    const req = await fetch(`${process.env.NERDTREE_API_URL}/post/update/new`, {
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
