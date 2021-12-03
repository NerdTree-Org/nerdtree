import NerdTreeSession from '~/store/session'
import { StatusPayload } from '~/api-wrapper/common'

export default async function RefreshAccessToken(
  ctx: NerdTreeSession
): Promise<StatusPayload> {
  const refreshToken = ctx.RefreshToken;

  if (!refreshToken) {
    return {
      success: false,
      message: "User is not logged in!"
    }
  }

  const payload = {
    refresh_token: refreshToken
  };

  try {
    const req = await fetch(`${process.env.NERDTREE_API_URL}/auth/refresh_token`, {
      method: 'post',
      body: JSON.stringify(payload),
      headers: {
        'content-type': 'application/json'
      },
    });
    const jsonBody = await req.json();

    if (req.status !== 200) {
      return {
        success: false,
        message: jsonBody.error
      }
    }
    else {
      ctx.updateAccessToken(jsonBody.access_token);

      return {
        success: true
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
