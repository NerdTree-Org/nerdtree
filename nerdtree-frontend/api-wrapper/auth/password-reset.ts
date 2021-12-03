import { StatusPayload } from '~/api-wrapper/common'

export async function RequestPasswordReset(
  email: String
): Promise<StatusPayload> {
  const payload = {
    email,
  };

  try {
    const req = await fetch(`${process.env.NERDTREE_API_URL}/auth/password_reset/request`, {
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
        message: jsonBody.error,
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

export async function ResetPassword(
  resetToken: String,
  password: String,
): Promise<StatusPayload> {
  const payload = {
    reset_token: resetToken,
    password,
  };

  try {
    const req = await fetch(`${process.env.NERDTREE_API_URL}/auth/password_reset/token`, {
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
        message: jsonBody.error,
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
