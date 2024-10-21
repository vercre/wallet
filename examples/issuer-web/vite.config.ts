import react from '@vitejs/plugin-react'
import { defineConfig, normalizePath } from 'vite'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        react(),
    ],
    server: {
        host: '0.0.0.0',
        port: 3000,
    },
    // envDir: '../',
});
