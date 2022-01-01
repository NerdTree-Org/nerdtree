import NerdTreeSession from '~/store/session'
import { StatusPayload } from '~/api-wrapper/common'
import { Post } from '~/api-wrapper/blog'

// Note: the caller must ensure that body
// has post_id as first field and thumbnail as last
export default async function Thumbnail(
  body: FormData,
  ctx: NerdTreeSession
): Promise<StatusPayload<Post>> {
  try {
    const req = await fetch(
      `${process.env.NERDTREE_API_URL}/post/update/thumbnail`,
      {
        method: 'post',
        body,
        headers: {
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
