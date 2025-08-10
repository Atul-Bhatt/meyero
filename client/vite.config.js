import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
	  port: 3000,
    allowedHosts: ["23bd64cb6395.ngrok-free.app", "localhost:8081"]
  }
})
