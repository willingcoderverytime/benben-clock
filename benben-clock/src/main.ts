import "./assets/main.css";

import App from "./App.vue";
import { createApp } from "vue";
import Antd from "ant-design-vue";
import { createPinia } from "pinia";

const app = createApp(App);
const store = createPinia();
app.use(Antd);
app.use(store);
app.mount("#app");

const disableRefresh = () => {
  document.addEventListener("keydown", function (event) {
    if (
      event.key === "F5" ||
      (event.ctrlKey && event.key === "r") ||
      (event.metaKey && event.key === "r")
    ) {
      event.preventDefault();
    }
  });
  document.addEventListener("contextmenu", function (event) {
    event.preventDefault();
  });
};

disableRefresh();
