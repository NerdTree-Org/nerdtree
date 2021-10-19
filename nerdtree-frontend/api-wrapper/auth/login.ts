import Cookies from 'js-cookie';
import { StatusPayload } from '~/api-wrapper/common'
import NerdTreeSession  from '~/store/session';

// Tries to log in with credentials.
// If successful, it'll set refresh token and access token to
// where it needs to be set
export default async function Login(ctx: NerdTreeSession, username: string, password: string): Promise<StatusPayload> {
  const payload = {
    username,
    password
  };

  const req = await fetch(`${process.env.NERDTREE_API_URL}/auth/login`, {
    method: 'post',
    body: JSON.stringify(payload),
    headers: {
      'content-type': 'application/json'
    },
  });

  const body = await req.text();
  try {
    const jsonBody = JSON.parse(body);
    if (req.status !== 200) {
      return {
        success: false,
        message: jsonBody.error
      }
    } else {
      // save the tokens
      Cookies.set('NERDTREE_AUTH_REFRESHTOKEN', jsonBody.refreshtoken, {
        sameSite: 'strict',
        expires: Date.now() + 30
      });
      ctx.updateAccessToken(jsonBody.accesstoken);
      return {
        success: true
      }
    }
  }
  catch (e) {
    return {
      success: false,
      message: e
    }
  }
}
