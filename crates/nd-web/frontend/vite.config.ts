import { fileURLToPath, URL } from "node:url";
import vue from "@vitejs/plugin-vue";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";

// we set this in rust on debug when we spin up the vite process
// for release (in build.rs), we set it to an an empty string
let VITE_NATIVEDOCTOR_API_ENDPOINT =
  process.env.VITE_NATIVEDOCTOR_API_ENDPOINT ?? "http://127.0.0.1:8080";

export default defineConfig({
  plugins: [vue(), tailwindcss()],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  server: {
    port: 5173,
    proxy: {
      "/api": {
        target: VITE_NATIVEDOCTOR_API_ENDPOINT,
        changeOrigin: true,
      },
    },
  },
  build: {
    outDir: "dist",
    emptyOutDir: true,
  },
});
