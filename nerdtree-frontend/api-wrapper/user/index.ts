import Firstname from '~/api-wrapper/user/update/firstname'
import Lastname from '~/api-wrapper/user/update/lastname'
import Email from '~/api-wrapper/user/update/email'
import Password from '~/api-wrapper/user/update/password'
import ProfilePic from '~/api-wrapper/user/update/profile_pic'
import DeleteUser from '~/api-wrapper/user/update/delete'
import ById from '~/api-wrapper/user/query/by_id'
import ByEmail from '~/api-wrapper/user/query/by_email'
import ByUsername from '~/api-wrapper/user/query/by_username'
import Current from '~/api-wrapper/user/query/current'
import DiscordToken from '~/api-wrapper/user/query/discord_token'

export type User = {
  id: String
  username: String
  firstname: String
  lastname: String
  email: String
  // eslint-disable-next-line camelcase
  profile_pic: String | null
  // eslint-disable-next-line camelcase
  is_admin: boolean
  // eslint-disable-next-line camelcase
  facebook_id: String
  // eslint-disable-next-line camelcase
  discord_token: String
  // eslint-disable-next-line camelcase
  is_discord_token_used: boolean
}

export default {
  update: {
    Firstname,
    Lastname,
    Email,
    Password,
    ProfilePic,
    DeleteUser,
  },
  query: {
    ById,
    ByEmail,
    ByUsername,
    Current,
    DiscordToken,
  },
}
