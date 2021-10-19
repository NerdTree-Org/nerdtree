import { Module, Mutation, VuexModule } from 'vuex-module-decorators'

type User = {
  id: string,
  username: string,
  firstname: string,
  lastname: string,
  email: string,
  profilePic: string,
  facebookId: string,
  discordToken: string,
  isDiscordTokenUsed: boolean
}

@Module({
  name: 'NerdTreeSession',
  stateFactory: true,
  namespaced: true
})
export default class NerdTreeSession extends VuexModule {
  user: User | null = null;
  accessToken: String | null = null;

  @Mutation
  updateUser(user: User) {
    this.user = user;
  }

  @Mutation
  updateAccessToken(token: String) {
    this.accessToken = token;
  }

  get loggedIn(): boolean {
    return !!this.user;
  }

  get getUser(): User | null {
    return this.user;
  }

  get getAccessToken(): String | null {
    return this.accessToken;
  }
}
