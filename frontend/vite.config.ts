import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import type { UserConfig } from 'vite';

// https://vitejs.dev/config/
export default defineConfig(async (): Promise<UserConfig> => ({
  plugins: [react()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  clearScreen: false,
  server: {
    port: 3003,
    strictPort: false, // Erlaubt Fallback auf anderen Port
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.app/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM === "windows" ? ["chrome105"] : ["es2021", "chrome105", "safari13"],
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },
}));
