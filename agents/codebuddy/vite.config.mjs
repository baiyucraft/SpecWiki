import { builtinModules } from "node:module";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { defineConfig } from "vite";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const externalModules = new Set([
  ...builtinModules,
  ...builtinModules.map((moduleName) => `node:${moduleName}`),
]);

export default defineConfig({
  build: {
    // The Agent package is a Node-side library, so the bundle targets Node rather than browsers.
    target: "node18",
    outDir: "dist",
    emptyOutDir: true,
    sourcemap: true,
    lib: {
      entry: path.resolve(__dirname, "src", "index.ts"),
      formats: ["es"],
      fileName: () => "index.js",
    },
    rollupOptions: {
      // Node built-ins must stay external, otherwise the bundle would try to polyfill runtime APIs.
      external(id) {
        return externalModules.has(id) || id.startsWith("node:");
      },
    },
  },
  test: {
    environment: "node",
    include: ["src/**/*.test.ts"],
    testTimeout: 30000,
  },
});
