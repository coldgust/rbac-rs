import { createApp } from 'vue'
import { createPinia } from 'pinia'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import 'nprogress/nprogress.css'
import App from './App.vue'
import router from './router'
import i18n from './locales'
import { setupPermissionDirective } from './directives/permission'
import './styles/index.scss'

const app = createApp(App)

app.use(createPinia())
app.use(router)
app.use(i18n)
app.use(ElementPlus)
setupPermissionDirective(app)

app.mount('#app')
