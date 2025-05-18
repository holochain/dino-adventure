import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    maxConcurrency: 1, // run tests serially, multi-conductor tests max out the machine anyway
    fileParallelism: false,
    testTimeout: 60 * 1000 * 4, // 4  mins
  },
});
