<template>
 
  <!-- <p>{{ greetMsg }}</p> -->
  <img v-for="item in state.list" width="300" :src="item" alt="">
</template>

<script setup>
import { reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";
const state = reactive({
  list: []
})
async function greet() {

  state.list = await invoke("greet", {
    page: '1'
  });

}
greet()
</script>