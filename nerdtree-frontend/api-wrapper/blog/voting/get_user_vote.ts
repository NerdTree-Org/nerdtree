import NerdTreeSession from '~/store/session'
import { StatusPayload } from '~/api-wrapper/common'

export enum VoteType {
  upvote = 1,
  downvote = -1,
  none = 0,
}

type voteIndicator = {
  vote: VoteType
}

export default async function GetUserVoteForPost(
  postId: String,
  ctx: NerdTreeSession
): Promise<StatusPayload<voteIndicator>> {
  const payload = {
    post_id: postId,
  }

  try {
    const req = await fetch(
      `${process.env.NERDTREE_API_URL}/post/query/by_current_user`,
      {
        method: 'post',
        body: JSON.stringify(payload),
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
