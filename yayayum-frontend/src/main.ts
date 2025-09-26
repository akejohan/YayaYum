import { mount } from 'svelte'
import './app.css'
import App from './App.svelte'
import { OpenAPI } from './lib/api'

// Set backend URL
OpenAPI.BASE = 'http://localhost:3000';

const app = mount(App, {
  target: document.getElementById('app')!,
})

export default app
