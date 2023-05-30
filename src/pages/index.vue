<script setup lang="ts">
import { invoke } from "@tauri-apps/api";

const ping = async () => {
  try {
    const res = await invoke("get_random_data");
    console.log(res);
  } catch (e) {
    console.error("error:", e);
  }
};

</script>

<template>
  <h1 class="text-clip">Hello World !</h1>
  <button @click="ping">ping</button>

  <NuxtLink to="/chapters/1">Go to About</NuxtLink>
</template>
