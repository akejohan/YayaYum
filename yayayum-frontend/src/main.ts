import { mount } from 'svelte'
import './app.css'
import App from './App.svelte'
import { OpenAPI } from './lib/api'

// Set backend URL
export const API_URL = import.meta.env.VITE_API_URL;
OpenAPI.BASE = API_URL;

const app = mount(App, {
  target: document.getElementById('app')!,
})

export default app
