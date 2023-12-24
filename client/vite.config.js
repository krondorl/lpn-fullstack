import { defineConfig } from "vite";
import path from "node:path";

export default defineConfig({
  resolve: {
    alias: {
      js: path.resolve(__dirname, "./js"),
    },
  },
});
