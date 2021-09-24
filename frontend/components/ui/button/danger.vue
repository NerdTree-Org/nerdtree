<template>
  <button class="pl-5 pr-5 pt-2 pb-2 m-2 text-white font-bold bg-red-700 rounded-md danger-button focus:outline-none ring-red-900 focus:ring-4">
    <slot />
  </button>
</template>

<script>
import Vue from 'vue';

export default Vue.extend({});
</script>

<style scoped>
.danger-button {
  transition: background .5s, box-shadow .2s;
}
.danger-button:active {
  background: #dc2626;
}
</style>
