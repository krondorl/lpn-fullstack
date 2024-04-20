import { defineConfig } from "vite";
import path from "node:path";

export default defineConfig({
  resolve: {
    alias: {
      js: path.resolve(__dirname, "./js"),
    },
  },
  preview: {
    proxy: {
      "/api/lpn-calc": {
        target: "http://server:8080/api/lpn-calc",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ""),
      },
    },
    host: "0.0.0.0",
    port: 3000,
  },
  server: {
    proxy: {
      "/api/lpn-calc": {
        target: "http://localhost:8080/api/lpn-calc",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ""),
      },
    },
    host: "0.0.0.0",
    port: 3000,
  },
});
