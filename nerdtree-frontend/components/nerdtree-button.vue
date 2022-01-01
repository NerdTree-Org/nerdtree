<template>
  <button :class='`button-${buttonType}`' class='pl-8 pr-8 pt-2 pb-2 rounded-md m-3'>
    <slot />
  </button>
</template>

<script lang="ts">
import Vue from "vue";

export enum ButtonType {
  DefaultButton = "default",
  TransparentButton = "transparent",
  FoggyButton = "foggy",
  BorderButton = "with-border",
  SidebarButton = "sidebar",
  SidebarButtonActive = "sidebar-active",
  SidebarDangerButton = "sidebar-danger",
  SidebarDangerActiveButton = "sidebar-danger-active"
}

export default Vue.extend({
  props: {
    buttonType: {
      required: true,
      type: () => ButtonType,
      default: ButtonType.DefaultButton
    }
  }
});
</script>

<style scoped>
.button-default {
  background: linear-gradient(102.61deg, #4B4B4B 25.57%, rgba(75, 75, 75, 0) 108.53%);
}

.button-default, .button-with-border, .button-transparent, .button-foggy {
  transition: box-shadow 200ms;
}

.button-default:active, .button-with-border:active, .button-transparent:active, .button-foggy:active {
  @apply ring-4;
  @apply ring-gray;
}

.button-with-border {
  background: linear-gradient(95.48deg, #28292C 0%, #131415 100%);
  border: 1px solid rgba(255, 255, 255, 0.12);
}

.button-foggy {
  background: linear-gradient(34.96deg, #C4C4C4 -204.1%, rgba(196, 196, 196, 0) 149.2%);
}

.button-sidebar {
  border-radius: 0;
  margin: 0;
  text-align: left;
}
.button-sidebar-active {
  border-radius: 0;
  margin: 0;
  background: #0C0C0C;
  text-align: left;
}
.button-sidebar-danger {
  border-radius: 0;
  margin: 0;
  color: #FF3232;
  text-align: left;
}
.button-sidebar-danger-active {
  border-radius: 0;
  margin: 0;
  color: #FF3232;
  background: #0C0C0C;
  text-align: left;
}

.button-sidebar:active, .button-sidebar-danger:active {
  background: #505050;
}

.button-sidebar-active:active, .button-sidebar-danger-active:active {
  background: #131313;
}
</style>
