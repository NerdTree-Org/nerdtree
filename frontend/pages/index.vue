<template>
  <div class="bg-black text-white h-screen">
    <h1 class="font-sans">
      NerdTree Web Infrastructure
    </h1>
    <h2>
      NerdTree UI
    </h2>
    <div>
      <NormalButton>Normal Button</NormalButton>
      <TransparentButton>Transparent Button</TransparentButton>
      <BorderlessButton>Borderless Button</BorderlessButton>
      <DangerButton>Danger Button</DangerButton>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import { NormalButton, TransparentButton, DangerButton, BorderlessButton } from '@/components/ui/button';

export default Vue.extend({
  components: {
    TransparentButton,
    NormalButton,
    BorderlessButton,
    DangerButton
  }
})
</script>
