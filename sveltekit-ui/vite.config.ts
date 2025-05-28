import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [
		sveltekit(),
	],
	// // This part is kept for history/documentation purposes.
	// // It was useful when staring nginx reverse proxying through docker.
	// server: {
	//     allowedHosts: [
	//     	'localhost',
	//     	'host.docker.internal'
	//     ],
	// },
});
