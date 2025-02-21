import tailwindcss from "@tailwindcss/vite";
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit(), tailwindcss()],
	server: {
		allowedHosts: ["bc69-102-89-83-197.ngrok-free.app"],
		fs: {
			allow: ["."],
		},
	},
});
