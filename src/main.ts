import { createPinia } from "pinia";
import { createApp } from "vue";
import App from "./components/App.vue";
import "./style.css";
import "tippy.js/dist/tippy.css";

createApp(App).use(createPinia()).mount("#app");

// disable the right click menu
document.addEventListener("contextmenu", (e) => e.preventDefault());
