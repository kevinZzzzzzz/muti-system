import { createApp } from "vue"
import App from "./App.vue"
import { invoke } from "@tauri-apps/api/core"

// const invoke = window.__TAURI__.core.invoke
invoke("my_custom_command", { invokeMessage: "hello from invoke" })
invoke("greet", { name: "Bob" })
  .then((response) => console.log(response))
  .catch((error) => console.error(error))
invoke("get_printer_list")
  .then((response) => console.log(response))
  .catch((error) => console.error(error))
createApp(App).mount("#app")
