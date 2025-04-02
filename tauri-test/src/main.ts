import { createApp } from "vue"
import App from "./App.vue"
// element css
import "element-plus/dist/index.css"
// element dark css
import "element-plus/theme-chalk/dark/css-vars.css"
import { invoke } from "@tauri-apps/api/core"
// element plus
import ElementPlus from "element-plus"
// element icons
// import * as Icons from "@element-plus/icons-vue"

// const invoke = window.__TAURI__.core.invoke
invoke("my_custom_command", { invokeMessage: "hello from invoke" })
// invoke("greet", { name: "Bob" })
//   .then((response: any) => console.log(response))
//   .catch((error: any) => console.error(error))
// invoke("get_printer_list")
//   .then((response: any) => console.log(response))
//   .catch((error: any) => console.error(error))
createApp(App).use(ElementPlus).mount("#app")
