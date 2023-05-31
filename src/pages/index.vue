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
  <h1>Page Chapters</h1>

  <NuxtLink to="/chapters/1">Go to About</NuxtLink>
</template>
