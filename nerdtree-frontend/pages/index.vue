<template>
  <div class='h-screen w-screen banner'>
    <form @submit='handleFormSubmit'>
      <input v-model='username' name='username' type='text' placeholder='username'/>
      <input v-model='password' name='password' type='password' placeholder='password'/>
      <NerdtreeButton button-type='DefaultButton'>Log In</NerdtreeButton>
      <p>Status: {{status}}</p>
    </form>
  </div>
</template>

<script lang="ts">
import Vue from 'vue';
import NerdtreeButton from '~/components/nerdtree-button.vue'
import NerdTreeAPI from '~/api-wrapper';
import { nerdtreeSession } from '~/utils/session-store-accessor'

export default Vue.extend({
  components: { NerdtreeButton },
  data() {
    return {
      username: '',
      password: '',
      status: {}
    }
  },
  methods: {
    async handleFormSubmit(e: Event) {
      e.preventDefault();
      this.status = await NerdTreeAPI.auth.Login(nerdtreeSession, this.username, this.password);
    }
  }
})
</script>

<style scoped>
.banner {
  background: rgb(24,24,24);
  background: linear-gradient(180deg, rgba(24,24,24,1) 0%, rgba(103,103,103,0) 100%);
}
</style>
