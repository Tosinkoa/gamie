import tailwindcss from "@tailwindcss/vite";
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit(), tailwindcss()],
	server: {
		allowedHosts: ["7f99-102-89-84-160.ngrok-free.app"],
		fs: {
			allow: ["."],
		},
	},
});
