<script setup lang="ts">
type Scene = {
  div: HTMLElement;
  x: number;
  y: number;
};

let offsetX = 0;
let offsetY = 0;
let isPanning = false;
let lastMouseX = 0;
let lastMouseY = 0;
let scenes: Scene[] = [];

function setCanvasSize(
  context: CanvasRenderingContext2D | null,
  canvas: HTMLCanvasElement
) {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;

  redrawCanvas(context, canvas);
}
function handleMouseDown(event: MouseEvent) {
  isPanning = true;
  lastMouseX = event.clientX;
  lastMouseY = event.clientY;

  console.log(isPanning);
}

function handleContextMenu(
  event: MouseEvent,
  context: CanvasRenderingContext2D | null,
  canvas: HTMLCanvasElement
) {
  event.preventDefault(); // Prevent the default context menu

  const sceneTemplate = `
    <div class="relative w-[250px] h-[150px] bg-blue-500 top-0 rounded-lg shadow-lg">
      <span class="w-[40px] h-[40px] absolute bg-black bottom-0 right-0 rounded-full"></span>
    </div>
  `;

  // Create scene element using the template
  const sceneElement = document.createElement('div');
  sceneElement.innerHTML = sceneTemplate.trim();

  // Get the mouse position relative to the canvas
  const rect = canvas.getBoundingClientRect();
  const mouseX = event.clientX - rect.left;
  const mouseY = event.clientY - rect.top;

  // Create a new scene at the mouse position
  const newScene = {
    div: sceneElement,
    x: mouseX,
    y: mouseY,
  }

  scenes.push(newScene)
  console.log(scenes)

  // Redraw the canvas to display the updated scenes
  redrawCanvas(context, canvas);
}

function handleMouseMove(
  event: MouseEvent,
  context: CanvasRenderingContext2D | null,
  canvas: HTMLCanvasElement
) {
  if (isPanning) {
    const deltaX = event.clientX - lastMouseX;
    const deltaY = event.clientY - lastMouseY;
    offsetX += deltaX;
    offsetY += deltaY;
    lastMouseX = event.clientX;
    lastMouseY = event.clientY;
    redrawCanvas(context, canvas);
  }
}

function handleMouseUp() {
  isPanning = false;
}

function redrawCanvas(
  context: CanvasRenderingContext2D | null,
  canvas: HTMLCanvasElement
) {
  // Draw objects on the canvas
  if (context) {
    context.clearRect(0, 0, canvas.width, canvas.height);
    context.translate(offsetX, offsetY);

    scenes.forEach((scene) => {
      const { div, x, y } = scene;
      div.style.transform = `translate(${x}px, ${y}px)`;
      canvas.appendChild(div);
    });

    context.translate(-offsetX, -offsetY);
  }

  // ...
}

onMounted(() => {
  const canvas = document.getElementById("canvaschapter") as HTMLCanvasElement;
  const context = canvas?.getContext("2d");

  setCanvasSize(context, canvas);
  window.addEventListener("resize", () => setCanvasSize(context, canvas));
  canvas.addEventListener("mousedown", handleMouseDown);
  canvas.addEventListener("mousemove", (e) =>
    handleMouseMove(e, context, canvas)
  );
  canvas.addEventListener("mouseup", handleMouseUp);
  canvas.addEventListener("mouseleave", handleMouseUp);
  canvas.addEventListener("contextmenu", (e) =>
    handleContextMenu(e, context, canvas)
  );
});

onBeforeUnmount(() => {
  const canvas = document.getElementById("canvaschapter") as HTMLCanvasElement;
  const context = canvas?.getContext("2d");

  window.removeEventListener("resize", () => setCanvasSize(context, canvas));
  canvas.removeEventListener("mousedown", handleMouseDown);
  canvas.removeEventListener("mousemove", (e) =>
    handleContextMenu(e, context, canvas)
  );
  canvas.removeEventListener("mouseup", handleMouseUp);
  canvas.removeEventListener("mouseleave", handleMouseUp);
  canvas.removeEventListener("contextmenu", (e) =>
    handleMouseMove(e, context, canvas)
  );
});
</script>

<template>
  <NuxtLink to="/" class="absolute top-0 left-0 text-white">Back home</NuxtLink>
  <canvas id="canvaschapter"></canvas>

</template>

<style>
#canvaschapter {
  background-color: rgb(14, 23, 31);
}
</style>
