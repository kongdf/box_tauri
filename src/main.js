import { createApp } from "vue";
import App from "./App.vue";
import router from "@/plugins/router";
import "@/assets/base.scss";
import "element-plus/theme-chalk/dark/css-vars.css";

let store = reactive({
    userThem: localStorage.getItem("useDarkKEY") || 'dark',
});
 

createApp(App).use(router).provide("store", store).mount("#app");

 