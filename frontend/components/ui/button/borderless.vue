<template>
  <button class="pl-5 pr-5 pt-2 pb-2 m-2 text-white font-bold borderless-button rounded-md focus:outline-none focus:ring-4 ring-gray-600">
    <slot />
  </button>
</template>

<script>
import Vue from 'vue';

export default Vue.extend({});
</script>

<style scoped>
.borderless-button {
  background: rgb(255,255,255);
  background: linear-gradient(90deg, rgba(255, 255, 255, 0.57) 0%, rgba(236, 230, 230, 0.4) 100%);
  background-size: 150%;
  transition: background-position .5s, box-shadow .2s;
}

.borderless-button:active {
  background-position-x: 100%;
}
</style>
