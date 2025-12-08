import path from "path";
import { defineConfig } from "vite";

export default defineConfig({
  base: "/assets/",
  build: {
    assetsDir: "",
    emptyOutDir: true,
    manifest: true,
    minify: true,
    modulePreload: {
      polyfill: false,
    },
    outDir: "../public/assets",
    rollupOptions: {
      input: "./src/main.js",
      output: {
        sourcemapPathTransform: (relativeSourcePath, _sourcemapPath) => {
          return path.relative("../../frontend", relativeSourcePath);
        },
      },
    },
    sourcemap: true,
  },
  server: {
    origin: "http://localhost:5173",
    cors: {
      origin: true,
    },
  },
});
