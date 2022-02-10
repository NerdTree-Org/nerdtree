import { StatusPayload } from '~/api-wrapper/common'

export type Votes = {
  votes: Number
}

export default async function GetVotes(
  postId: String
): Promise<StatusPayload<Votes>> {
  const payload = {
    post_id: postId,
  }

  try {
    const req = await fetch(`${process.env.NERDTREE_API_URL}/post/vote/votes`, {
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
      }
    }
  } catch {
    return {
      success: false,
      message: 'Cannot connect to server',
    }
  }
}
