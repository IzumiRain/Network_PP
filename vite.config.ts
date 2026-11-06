import { defineConfig } from "vite";

// Tauri expects fixed dev port and dist output at ../dist (relative to src-tauri).
export default defineConfig({
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    // Windows: cargo locks exe files under src-tauri/target while compiling;
    // watching them makes the dev server die with EBUSY.
    watch: {
      ignored: ["**/src-tauri/target/**", "**/dist/**"],
    },
  },
  build: {
    target: "es2022",
    outDir: "dist",
    emptyOutDir: true,
    // Weak-device target: keep chunks small, drop modulepreload polyfill weight.
    modulePreload: false,
    reportCompressedSize: false,
  },
});
