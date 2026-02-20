import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import tailwindcss from "@tailwindcss/vite";
import type { UserConfig } from "vite";

const __dirname = dirname(fileURLToPath(import.meta.url));

export default {
	root: "./vite",
	plugins: [tailwindcss()],
	build: {
		outDir: "../dist",
		emptyOutDir: true,
		rollupOptions: {
			input: {
				main: resolve(__dirname, "./vite/index.html"),
				"404": resolve(__dirname, "./vite/404.html"),
				"500": resolve(__dirname, "./vite/500.html"),
			},
		},
	},
} satisfies UserConfig;
