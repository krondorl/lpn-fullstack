import { defineConfig } from "vite";
import path from "node:path";

export default defineConfig({
  resolve: {
    alias: {
      js: path.resolve(__dirname, "./js"),
    },
  },
  preview: {
    host: "0.0.0.0",
    port: 3000,
  },
  server: {
    host: "0.0.0.0",
    port: 3000,
  },
});
