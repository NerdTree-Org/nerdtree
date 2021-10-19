<template>
  <div class='blog-post-card flex flex-row p1 pt-2 m-2'>
    <div class='m-5 mr-0 text-xl flex flex-col'>
      <!-- Upvotes stuff -->
      <span class='p-2 vote-button text-center rounded'>↑</span>
      <span class='p-2 text-center'>30</span>
      <span class='p-2 vote-button text-center rounded'>↓</span>
    </div>
    <div class='m-5 ml-3'>
      <!-- Blog info -->
      <h3 class='text-xl mb-3' style='color: #BDBDBD'>u/mdgaziur001</h3>
      <nuxt-link :to='`/post/${postId}`'>
        <h1 class='text-2xl overflow-ellipsis blog-title'>
          Test blog for testing purpose
        </h1>
      </nuxt-link>
      <h3 class='mt-2 text-xl' style='color: #D8D8D8'>5 min read time</h3>
      <img class='blog-thumbnail mt-3 max-w-full' src='https://source.unsplash.com/random/300x300' alt='Blog thumbnail'>
    </div>
  </div>
</template>

<script>
import Vue from 'vue';

export default Vue.extend({
  props: {
    postId: {
      type: String,
      required: true
    }
  }
});
</script>

<style scoped>
.blog-post-card {
  max-width: 400px;
  background: linear-gradient(223.22deg, rgba(109, 109, 109, 0.42) -23.79%, rgba(255, 255, 255, 0) 100.29%);
  border-radius: 15px;
  border: 1px solid rgba(255, 255, 255, 0.24);
}

.blog-title:hover {
  color: #8f8f8f;
}

.blog-thumbnail {
  border-radius: 15px;
}

.vote-button {
  cursor: pointer;
}

.vote-button:hover {
  background: rgba(0, 0, 0, 0.26);
}

.vote-button:active {
   background: rgba(0, 0, 0, 0.53);
 }
</style>
