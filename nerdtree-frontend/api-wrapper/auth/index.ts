import Register from '~/api-wrapper/auth/register'
import RefreshAccessToken from '~/api-wrapper/auth/refresh-accesstoken'
import { RequestPasswordReset, ResetPassword } from '~/api-wrapper/auth/password-reset'
import Login from '~/api-wrapper/auth/login'

export default {
  Login,
  Register,
  RefreshAccessToken,
  RequestPasswordReset,
  ResetPassword
}
