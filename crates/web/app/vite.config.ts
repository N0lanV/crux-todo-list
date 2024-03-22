import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { VitePWA } from 'vite-plugin-pwa'

// https://vitejs.dev/config/
export default defineConfig({
        plugins: [
                react(),
                wasm(),
                topLevelAwait(),
                VitePWA({
                        registerType: 'autoUpdate',
                        devOptions: {
                                enabled: true
                        }
                })
        ],
})
