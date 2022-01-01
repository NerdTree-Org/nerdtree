import { Store } from 'vuex'
import { getModule } from 'vuex-module-decorators'
import NerdTreeSession from '~/store/session'

let nerdtreeSession: NerdTreeSession

function initialiseStores(store: Store<any>): void {
  nerdtreeSession = getModule(NerdTreeSession, store)
}

export { initialiseStores, nerdtreeSession }
