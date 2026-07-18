import { createApp } from 'vue'
import { createPinia } from 'pinia'
import 'vue-sonner/style.css'
import './index.css'
import '@/theme/echarts/theme.ts'
import App from './App.vue'

const app = createApp(App)

app.use(createPinia())
app.mount('#app')
