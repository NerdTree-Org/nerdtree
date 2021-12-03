import { StatusPayload } from '~/api-wrapper/common'
import NerdTreeSession  from '~/store/session'

// Tries to log in with credentials.
// If successful, it'll set refresh token and access token to
// where it needs to be set
export default async function Login(ctx: NerdTreeSession, username: string, password: string): Promise<StatusPayload> {
  const payload = {
    username,
    password
  };

  try {
    const req = await fetch(`${process.env.NERDTREE_API_URL}/auth/login`, {
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
    } else {
      // save the tokens
      ctx.updateRefreshToken(jsonBody.refreshtoken);
      ctx.updateAccessToken(jsonBody.accesstoken);
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
