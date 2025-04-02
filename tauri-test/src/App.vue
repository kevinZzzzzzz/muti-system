<template>
  <main class="container">
    <h1>Welcome to Tauri + Vue</h1>

    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>
    <div class="block1">
      <el-button type="success" @click="handleMsg">消息提示</el-button>
    </div>
    <br>
    <div class="block2">
      <el-select v-model="printer" placeholder="请选择打印机" style="width: 200px">
        <el-option v-for="item in printersList" :key="item.name" :label="item.name" :value="item.name">
        </el-option>
      </el-select>
      <el-button type="primary">打印</el-button>
    </div>
  </main>
</template>
<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { isPermissionGranted, sendNotification, requestPermission } from '@tauri-apps/plugin-notification';

const greetMsg = ref<string>("");
const name = ref<string>("");
const printersList = ref<any>([]);
const printer = ref<string>('');

async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value }) as string;
}
async function getPrintersList() {
  printersList.value = await invoke("get_printer_list");
  printer.value = printersList.value.filter((d: any) => { return !!d.is_default })[0].name
  console.log(printersList.value, 'printersList.value0000000000000')
}
const handleMsg = async () => {
  // const activeNotifications = await active();
  // console.log(123123, activeNotifications)
  sendNotification({
    title: 'New Image',
    body: 'Check out this picture'
  });
}
onMounted(async () => {
  getPrintersList()
  let permissionGranted = await isPermissionGranted();
  if (!permissionGranted) {
    const permission = await requestPermission();
    permissionGranted = permission === 'granted';
  }

  console.log(permissionGranted, 'permissionGranted')
})
</script>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}
</style>