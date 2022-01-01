import { StatusPayload } from '~/api-wrapper/common'

export default async function Register(
  firstName: String,
  lastName: String,
  username: String,
  email: String,
  password: String,
  facebookId: String,
): Promise<StatusPayload> {
  const payload = {
    first_name: firstName,
    last_name: lastName,
    username,
    email,
    password,
    facebook_id: facebookId,
  };
  try {
    const req = await fetch(`${process.env.NERDTREE_API_URL}/auth/register`, {
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
      return {
        success: true,
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
