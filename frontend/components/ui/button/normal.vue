<template>
  <button class="pl-5 pr-5 pt-2 pb-2 m-2 text-white font-bold normal-button rounded-md focus:outline-none focus:ring-4 ring-gray-800">
    <slot />
  </button>
</template>

<style scoped>
.normal-button {
  background: rgb(47,47,47);
  background: linear-gradient(90deg, rgba(47,47,47,1) 0%, rgb(0, 0, 0) 100%);
  border: 1px solid rgba(166, 160, 160, 0.52);
  transition: background-position .5s, box-shadow .2s;
  background-position-x: 10%;
  background-size: 200%;
}

.normal-button:active {
  background-position-x: 100%;
}
</style>

<script>
import Vue from 'vue';

export default Vue.extend({});
</script>
