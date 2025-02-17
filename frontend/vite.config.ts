import tailwindcss from "@tailwindcss/vite";
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit(), tailwindcss()],
	server: {
		allowedHosts: ['6446-102-89-68-250.ngrok-free.app'],
		fs: {
			allow: ['.'],
		},
	},
});
