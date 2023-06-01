<script setup lang="ts">
const counter = useCounterForScene();

</script>

<template>
  <h1 class="text-7xl pl-[50px] pt-[50px] font-extrabold">Routes connected to</h1>
  <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 p-[50px] gap-6">
    <RouteCard sceneId="1" />
    <RouteCard sceneId="2"/>
  </div>
</template>