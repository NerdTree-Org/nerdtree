<template>
  <div class="min-h-screen grid">
    <div class="max-w-8xl mx-auto px-4 sm:px-6 md:px-8 grid">
      <div class="flex flex-col">
        <div class="flex sm:justify-end justify-center">
          <NuxtLink to="/login">
            <NerdtreeButton :button-type="ButtonType.TransparentButton">
              <b>Log In</b>
            </NerdtreeButton>
          </NuxtLink>
          <NuxtLink to="/register">
            <NerdtreeButton :button-type="ButtonType.DefaultButton">
              <b>Register</b>
            </NerdtreeButton>
          </NuxtLink>
        </div>
        <div class="flex flex-col justify-center h-full">
          <div class="flex flex-wrap justify-around flex-1">
            <div class="flex flex-col justify-center m-5 sm:m-0">
              <h1
                class="text-4xl sm:text-6xl font-header font-semibold tracking-tighter"
              >
                We're NerdTree
              </h1>
              <p
                class="text-xl sm:text-2xl max-w-xl mt-5 font-semibold tracking-tighter"
              >
                An organization aiming to make a better programming community
                for students
              </p>
            </div>
            <div class="flex flex-col justify-center items-center sm:items-end">
              <img
                class="rounded-full w-1/2"
                src="~/assets/nerdtree.png"
                alt="nerdtree logo"
              />
            </div>
          </div>
        </div>
        <div class="flex justify-center">
          <div
            class="flex justify-center flex-col flex-1 sm:flex-row block p-3 pt-3.5 pb-3.5 mt-3 mb-3 w-max rounded-xl text-xl font-semibold tracking-tight"
            style="
              background: #343434;
              border: 1px solid rgba(255, 255, 255, 0.16);
            "
          >
            <span
              class="select-none pl-4 pr-4 p-2 rounded-xl"
              style="
                background: #646464;
                border: 1px solid rgba(255, 255, 255, 0.16);
              "
              >/</span
            >
            <NuxtLink
              class="nav-link mt-2 sm:mt-0 pl-7 pr-7 pt-2 pb-2 mt-0 rounded-xl sm:ml-2 ml-0"
              to="/blogs"
            >
              Blogs
            </NuxtLink>
            <NuxtLink
              class="nav-link mt-2 sm:mt-0 sm:ml-2 ml-0 pl-7 pr-7 pt-2 pb-2 rounded-xl"
              to="/workshops"
            >
              Workshops
            </NuxtLink>
            <NuxtLink
              class="nav-link mt-2 sm:mt-0 sm:ml-2 ml-0 pl-7 pr-7 pt-2 pb-2 rounded-xl"
              to="/about"
            >
              About Us
            </NuxtLink>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
* {
  color: white;
}

.nav-link {
  border: 1px solid transparent;
  user-select: none;
  cursor: pointer;
  transition: background 0.1s;
}

.nav-link:hover {
  background: #525252;
  border-color: rgba(255, 255, 255, 0.16);
}

.nav-link:active {
  background: #646464;
}
</style>

<script lang="ts">
import Vue from 'vue'
import { ButtonType } from '~/components/nerdtree-button.vue'

export default Vue.extend({
  name: 'IndexPage',
  data() {
    return {
      ButtonType,
    }
  },
})
</script>
