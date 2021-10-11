<template>
  <div>
    <h1>NerdTree Web</h1>
    <NerdTreeButton  :button-type='ButtonType.DefaultButton'>Default button</NerdTreeButton>
    <NerdTreeButton  :button-type='ButtonType.TransparentButton'>Transparent button</NerdTreeButton>
    <NerdTreeButton  :button-type='ButtonType.BorderButton'>Border button</NerdTreeButton>
    <NerdTreeButton  :button-type='ButtonType.FoggyButton'>Foggy button</NerdTreeButton>
    <div class='flex flex-col content-start w-max' style='background: linear-gradient(180deg, #232323 10%, #0C0C0C 120%); height: 600px'>
      <NerdTreeButton  :button-type='ButtonType.SidebarButton'>Sidebar button</NerdTreeButton>
      <NerdTreeButton  :button-type='ButtonType.SidebarButtonActive'>Sidebar active button</NerdTreeButton>
      <NerdTreeButton  :button-type='ButtonType.SidebarDangerButton'>Sidebar danger button</NerdTreeButton>
      <NerdTreeButton  :button-type='ButtonType.SidebarDangerActiveButton'>Sidebar danger active button</NerdTreeButton>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import NerdTreeButton, { ButtonType } from '~/components/nerdtree-button.vue';

export default Vue.extend({
  components: {
    NerdTreeButton
  },
  data() {
    return {
      ButtonType
    }
  }
})
</script>
