<template>
  <button
    class="pl-5 pr-5 pt-2 pb-2 m-2 text-white font-bold transparent-button rounded-md focus:ring-4 focus:outline-none ring-gray-800 focus:bg-gray-600"
  >
    <slot/>
  </button>
</template>

<script lang="ts">
import Vue from 'vue'

export default Vue.extend({})
</script>

<style scoped>
.transparent-button {
  transition: background .5s, box-shadow .2s;
}
</style>
