import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vite";
import tailwindcss from "@tailwindcss/vite";

// https://vitejs.dev/config/
export default defineConfig({
/* eslint-disable */
plugins: [svelte(), tailwindcss()],
/* eslint-enable */
});
