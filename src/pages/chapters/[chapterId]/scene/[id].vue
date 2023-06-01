<script lang="ts" setup>
const images = useImages();
const route = useRoute();
const routeId = route.params.id as unknown as number;
let imageToShow = ref(0);

import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

listen("tauri://file-drop", async (event) => {
  const payload = event.payload as Array<string>;
  const path = payload[0] as string;
  const image = (await invoke("image_to_base64", { path: path })) as string;

  if (images.value.find((image) => image.id === routeId)) {
    images.value.find((image) => image.id === routeId)?.images.push(image);
  } else {
    images.value.push({ id: routeId, images: [image] });
  }
});

function handleDragOver(ev: DragEvent) {
  ev.preventDefault();
}

function handleDrop(ev: DragEvent) {
  ev.preventDefault();

  const file = ev.dataTransfer?.files[0];
  if (file) {
    const reader = new FileReader();

    reader.onload = () => {
      if (images.value.find((image) => image.id === routeId)) {
        images.value
          .find((image) => image.id === routeId)
          ?.images.push(reader.result as string);
      } else {
        images.value.push({ id: routeId, images: [reader.result as string] });
      }

      console.log(images.value);
    };

    reader.readAsDataURL(file);
  }
}

function handleClick() {
  if(images.value.length > imageToShow.value) {
    imageToShow.value++
  }
}

onMounted(() => {
  const dropArea = document.getElementById("dropArea");

  if (dropArea) {
    dropArea.addEventListener("dragover", handleDragOver);
    dropArea.addEventListener("drop", handleDrop);
  }
});
</script>

<template>
  <h1
    :style="{ display: images[0] ? 'none' : '' }"
    class="absolute text-[2.5vw] text-center translate-x-[-50%] translate-y-[-50%] top-[45%] left-[50%]"
  >
    Drag and drop the background image for your scene
  </h1>
  <div
    id="dropArea"
    @click="handleClick"
    class="flex flex-col items-center justify-end w-screen h-screen pb-[100px] bg-contain bg-no-repeat bg-center"
    :style="{
      backgroundImage: `url(${
        images.find((image) => image.id === routeId)?.images[imageToShow]
      })`,
    }"
  >
    <div class="flex flex-col w-[50vw] 2xl:gap-[2vh]">
      <h2 class="text-[4vh] xl:text-[4vh] 2xl:text-5xl font-medium">Name</h2>
      <p class="text-base xl:text-[3vh] 2xl:text-2xl">
        Lorem ipsum dolor sit amet consectetur adipisicing elit. Rerum nemo quia
        nulla deserunt sint? Provident quidem temporibus odit natus quia,
        ratione mollitia ad tempora porro officiis, voluptatum inventore
        explicabo dolor?
      </p>
    </div>
  </div>
</template>
