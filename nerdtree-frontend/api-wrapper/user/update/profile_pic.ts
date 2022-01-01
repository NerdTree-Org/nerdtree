import NerdTreeSession from '~/store/session'
import { StatusPayload } from '~/api-wrapper/common'

// The caller must ensure that the FormData
// only contains profile picture and it's the first field
export default async function ProfilePic(
  formData: FormData,
  ctx: NerdTreeSession
): Promise<StatusPayload> {
  try {
    const req = await fetch(
      `${process.env.NERDTREE_API_URL}/user/update/firstname`,
      {
        method: 'post',
        body: formData,
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
