import { createApp } from "vue";
import "./styles.scss";
import App from "./App.vue";
import { router } from './router/index'
import { createPinia } from 'pinia'

const pinia = createPinia()


createApp(App).use(router).use(pinia).mount("#app");
