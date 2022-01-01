import NerdTreeSession from '~/store/session'
import { StatusPayload } from '~/api-wrapper/common'
import { Post } from '~/api-wrapper/blog'

export default async function Title(
  postId: String,
  title: String,
  ctx: NerdTreeSession
): Promise<StatusPayload<Post>> {
  const payload = {
    post_id: postId,
    title,
  };

  try {
    const req = await fetch(`${process.env.NERDTREE_API_URL}/post/update/title`, {
      method: 'post',
      body: JSON.stringify(payload),
      headers: {
        'content-type': 'application/json',
        'authorization': `bearer ${ctx.AccessToken}`
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
