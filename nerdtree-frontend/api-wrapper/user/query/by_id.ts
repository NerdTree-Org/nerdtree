import { StatusPayload } from '~/api-wrapper/common'
import { User } from '~/api-wrapper/user'

export default async function ById(
  userId: String
): Promise<StatusPayload<User>> {
  const payload = {
    user_id: userId,
  }

  try {
    const req = await fetch(`${process.env.NERDTREE_API_URL}/user/query/id`, {
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
