import { defineConfig } from "vite"
import vue from "@vitejs/plugin-vue"

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  build: {
    // Tauri 在 Windows 上使用 Chromium，在 macOS 和 Linux 上使用 WebKit
    target: "chrome105",
    // 在 debug 构建中不使用 minify
    minify: false,
    // 在 debug 构建中生成 sourcemap
    sourcemap: false,
  },
}))
