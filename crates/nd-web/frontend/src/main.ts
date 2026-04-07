import EditorWorker from "monaco-editor/esm/vs/editor/editor.worker.js?worker";
import { createPinia } from "pinia";
import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";

/** Required for Monaco ESM in Vite (see monaco-editor docs / VS Code worker model). */
self.MonacoEnvironment = {
    getWorker(): Worker {
        return new EditorWorker();
    },
};

const app = createApp(App);
app.use(createPinia());
app.mount("#app");
