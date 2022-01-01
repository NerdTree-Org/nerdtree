import NerdTreeSession from '~/store/session'
import { StatusPayload } from '~/api-wrapper/common'
import { User } from '~/api-wrapper/user'

export default async function Current(
  ctx: NerdTreeSession
): Promise<StatusPayload<User>> {
  try {
    const req = await fetch(
      `${process.env.NERDTREE_API_URL}/user/query/current`,
      {
        method: 'post',
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
