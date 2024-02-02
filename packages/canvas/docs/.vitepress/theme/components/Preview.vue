<template>
  <div>
    <div class="flex flex-col justify-center items-center mt-8">
      <h3 class="text-xl font-bold">Preview with all flavors</h3>
      <div class="flex justify-center flex-wrap mt-4">
        <div class="btn js">
          <img style="height: 27px; width: 27px;"
            src="https://upload.wikimedia.org/wikipedia/commons/thumb/9/99/Unofficial_JavaScript_logo_2.svg/1200px-Unofficial_JavaScript_logo_2.svg.png" />
          <span class="ml-1">
            JS</span>
        </div>
        <div class="btn ts">
          <img style="height: 27px; width: 27px;"
            src="https://upload.wikimedia.org/wikipedia/commons/thumb/4/4c/Typescript_logo_2020.svg/1200px-Typescript_logo_2020.svg.png" />
          <span class="ml-1">
            TS</span>
        </div>
        <div class="btn vue">
          <img style="height: 27px; width: 27px;"
            src="https://upload.wikimedia.org/wikipedia/commons/thumb/9/95/Vue.js_Logo_2.svg/1184px-Vue.js_Logo_2.svg.png" />
          <span class="ml-1">
            Vue.js</span>
        </div>
        <div class="btn angular">
          <img style="height: 30px; width: 30px;"
            src="https://miro.medium.com/v2/resize:fit:2680/1*jlg3PXZ6PYdUGy40tXybKw.gif" />
          <span class="ml-1">
            Angular
          </span>
        </div>
        <div class="btn svelte">
          <img style="height: 28px; width: 22px;"
            src="https://upload.wikimedia.org/wikipedia/commons/thumb/1/1b/Svelte_Logo.svg/1200px-Svelte_Logo.svg.png" />
          <span class="ml-1">
            Svelte
          </span>
        </div>
        <div class="btn react">
          <img style="height: 28px; width: 32px;"
            src="https://upload.wikimedia.org/wikipedia/commons/thumb/a/a7/React-icon.svg/1024px-React-icon.svg.png" />
          <span class="ml-1">

            React</span>
        </div>
        <div class="btn solid">
          <img style="height: 28px; width: 28px;" src="https://www.solidjs.com/assets/logo-123b04bc.svg" />
          <span class="ml-1">
            Solid.js
          </span>
        </div>
      </div>


    </div>
    <div v-if="isChromium && width > 768" class="wrap-iframe mt-8">
      <iframe class="mt-4"
        src="https://stackblitz.com/edit/nativescrip-use-vue?embed=1&file=src%2Fapp.ts&view=both"></iframe>
    </div>
    <div v-else class=" flex justify-center mb-12">
      <a class="rounded-full text-white bg-ns-vueuse px-3 py-2" target="_blank"
        href="https://stackblitz.com/edit/nativescrip-use-vue?file=src%2Fapp.ts">Playground</a>
    </div>
  </div>
</template>
<script setup>
import { onBeforeMount, ref } from "vue"
import { useWindowSize } from '@vueuse/core'

const { width } = useWindowSize();
const isChromium = ref(true);
onBeforeMount(() => {
  isChromium.value = !!window.chrome
})
const updateCursor = ({ x, y }) => {
  document.documentElement.style.setProperty('--x', x)
  document.documentElement.style.setProperty('--y', y)
}

document.body.addEventListener('pointermove', updateCursor)
</script>
<style scoped>
.btn {
  --active: 0;
  padding: 8px 12px;
  position: relative;
  transition: background 0.1s;
  margin: 0 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-items: center;
  flex-direction: col;
  margin-top: 6px;

}

.btn:hover {
  --active: 1;
}

.btn:after {
  content: "";
  position: absolute;
  inset: 0;
  background:
    radial-gradient(circle at calc(var(--x) * 1px) calc(var(--y) * 1px), hsl(0 0% 100% / 0.15), transparent 15vmin);
  opacity: var(--active, 0);
  transition: opacity 0.2s;
  pointer-events: none;

}

.btn:before {
  content: "";
  position: absolute;
  inset: 0;
  pointer-events: none;
  mask:
    linear-gradient(white, white) 50% 0 / 100% 2px no-repeat,
    linear-gradient(white, white) 50% 100% / 100% 2px no-repeat,
    linear-gradient(white, white) 0 50% / 2px 100% no-repeat,
    linear-gradient(white, white) 100% 50% / 2px 100% no-repeat;
}

.btn.js:before {
  background:
    radial-gradient(circle at calc(var(--x) * 1px) calc(var(--y) * 1px), #f7e018, transparent 15vmin),
    transparent;
  background-attachment: fixed;
}

.btn.ts:before {
  background:
    radial-gradient(circle at calc(var(--x) * 1px) calc(var(--y) * 1px), #2e79c7, transparent 15vmin),
    transparent;
  background-attachment: fixed;
}

.btn.vue:before {
  background:
    radial-gradient(circle at calc(var(--x) * 1px) calc(var(--y) * 1px), #42b883, transparent 15vmin),
    transparent;
  background-attachment: fixed;
}

.btn.angular:before {
  background:
    radial-gradient(circle at calc(var(--x) * 1px) calc(var(--y) * 1px), #c51fc9, transparent 15vmin),
    transparent;
  background-attachment: fixed;
}

.btn.svelte:before {
  background:
    radial-gradient(circle at calc(var(--x) * 1px) calc(var(--y) * 1px), #ff3e00, transparent 15vmin),
    transparent;
  background-attachment: fixed;
}

.btn.react:before {
  background:
    radial-gradient(circle at calc(var(--x) * 1px) calc(var(--y) * 1px), #149eca, transparent 15vmin),
    transparent;
  background-attachment: fixed;
}

.btn.solid:before {
  background:
    radial-gradient(circle at calc(var(--x) * 1px) calc(var(--y) * 1px), #325c9b, transparent 15vmin),
    transparent;
  background-attachment: fixed;
}



.wrap-iframe {
  width: 80%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  margin: auto;
}

.wrap-iframe iframe {
  width: 100%;
  border: none;
  height: 800px;
  background: #1e1e20;
  border-radius: 16px;
  overflow: none;
}
</style>