import { StatusPayload } from '~/api-wrapper/common'
import { Post } from '~/api-wrapper/blog'

export default async function ById(
  postId: String
): Promise<StatusPayload<Post>> {
  const payload = {
    post_id: postId,
  }

  try {
    const req = await fetch(`${process.env.NERDTREE_API_URL}/post/query/id`, {
      method: 'post',
      body: JSON.stringify(payload),
      headers: {
        'content-type': 'application/json',
      },
    })

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
