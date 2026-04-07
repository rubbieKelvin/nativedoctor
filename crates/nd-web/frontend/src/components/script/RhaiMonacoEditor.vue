<script setup lang="ts">
import * as monaco from "monaco-editor";
import "monaco-editor/min/vs/editor/editor.main.css";
import {
    onBeforeUnmount,
    onMounted,
    ref,
    shallowRef,
    watch,
} from "vue";

const props = withDefaults(
    defineProps<{
        modelValue: string;
        language?: string;
    }>(),
    { language: "rust" },
);

const emit = defineEmits<{
    "update:modelValue": [value: string];
}>();

const containerRef = ref<HTMLDivElement | null>(null);
const editorRef = shallowRef<monaco.editor.IStandaloneCodeEditor | null>(null);

/** True while the latest model change came from the editor (avoid setValue loops). */
let syncingFromEditor = false;

function applyMonacoTheme() {
    const dark = document.documentElement.classList.contains("dark");
    monaco.editor.setTheme(dark ? "vs-dark" : "vs");
}

let themeObserver: MutationObserver | null = null;

onMounted(() => {
    const el = containerRef.value;
    if (!el) return;

    applyMonacoTheme();

    const editor = monaco.editor.create(el, {
        value: props.modelValue,
        language: props.language,
        automaticLayout: true,
        minimap: { enabled: false },
        fontSize: 12,
        lineNumbers: "on",
        scrollBeyondLastLine: false,
        wordWrap: "on",
        tabSize: 2,
        padding: { top: 8 },
    });

    editorRef.value = editor;

    editor.onDidChangeModelContent(() => {
        syncingFromEditor = true;
        emit("update:modelValue", editor.getValue());
        queueMicrotask(() => {
            syncingFromEditor = false;
        });
    });

    themeObserver = new MutationObserver(() => applyMonacoTheme());
    themeObserver.observe(document.documentElement, {
        attributes: true,
        attributeFilter: ["class"],
    });
});

watch(
    () => props.modelValue,
    (v) => {
        if (syncingFromEditor) return;
        const ed = editorRef.value;
        if (!ed) return;
        const cur = ed.getValue();
        if (cur !== v) ed.setValue(v);
    },
);

watch(
    () => props.language,
    (lang) => {
        const ed = editorRef.value;
        const model = ed?.getModel();
        if (!ed || !model) return;
        monaco.editor.setModelLanguage(model, lang);
    },
);

onBeforeUnmount(() => {
    themeObserver?.disconnect();
    themeObserver = null;
    editorRef.value?.dispose();
    editorRef.value = null;
});
</script>

<template>
    <div ref="containerRef" class="min-h-0 h-full w-full overflow-hidden" />
</template>
