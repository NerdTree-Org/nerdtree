import NerdTreeSession from '~/store/session'
import { StatusPayload } from '~/api-wrapper/common'

export default async function Password(
  newPassword: String,
  oldPassword: String,
  ctx: NerdTreeSession
): Promise<StatusPayload> {
  const payload = {
    newPassword,
    oldPassword
  }

  try {
    const req = await fetch(`${process.env.NERDTREE_API_URL}/user/update/password`, {
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
