import { Module, Mutation, VuexModule } from 'vuex-module-decorators'
import Cookies from 'js-cookie'
import { User } from '~/api-wrapper/user'

@Module({
  name: 'NerdTreeSession',
  stateFactory: true,
  namespaced: true,
})
export default class NerdTreeSession extends VuexModule {
  private user: User | null = null
  private accessToken: String | null = null

  @Mutation
  updateUser(user: User) {
    this.user = user
  }

  @Mutation
  updateAccessToken(token: String) {
    this.accessToken = token
  }

  @Mutation
  updateRefreshToken(refreshToken: String) {
    Cookies.set('NERDTREE_AUTH_REFRESHTOKEN', refreshToken, {
      sameSite: 'strict',
      expires: Date.now() + 30,
    })
  }

  get loggedIn(): boolean {
    return !!this.user
  }

  get User(): User | null {
    return this.user
  }

  get AccessToken(): String | null {
    return this.accessToken
  }

  get RefreshToken(): String | null {
    const token = Cookies.get('NERDTREE_AUTH_REFRESHTOKEN')

    return token || null
  }
}
