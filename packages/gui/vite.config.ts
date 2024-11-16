import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [sveltekit()],
  clearScreen: false,
  server: {
    host: host || false,
    port: 1420,
    strictPort: true,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
  },
});
