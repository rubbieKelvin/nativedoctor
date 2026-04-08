<script setup lang="ts">
import { json } from "@codemirror/lang-json";
import { rhai } from "@/codemirror/rhai";
import { bracketMatching } from "@codemirror/language";
import type { Extension } from "@codemirror/state";
import { EditorState } from "@codemirror/state";
import { EditorView, minimalSetup } from "codemirror";
import { onBeforeUnmount, onMounted, ref, shallowRef, watch } from "vue";

const props = withDefaults(
    defineProps<{
        modelValue?: string;
        language?: "plaintext" | "json" | "rhai";
        /** Extra classes on the outer wrapper (e.g. min-height for body editors). */
        inputClass?: string;
    }>(),
    {
        modelValue: "",
        language: "plaintext",
        inputClass: "",
    },
);

const emit = defineEmits<{
    (e: "update:modelValue", v: string): void;
    (e: "blur"): void;
}>();

const host = ref<HTMLDivElement | null>(null);
const viewRef = shallowRef<EditorView | null>(null);

let applyingRemote = false;

const ndTheme = EditorView.theme({
    "&": {
        height: "100%",
        backgroundColor: "hsl(var(--background))",
    },
    ".cm-scroller": {
        overflow: "auto",
        fontFamily: "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace",
        fontSize: "12px",
        lineHeight: "1.45",
    },
    ".cm-content": {
        caretColor: "hsl(var(--foreground))",
        padding: "12px 0",
        minHeight: "100%",
    },
    ".cm-cursor, .cm-dropCursor": {
        borderLeftColor: "hsl(var(--foreground))",
    },
    "&.cm-focused .cm-selectionBackground, .cm-selectionBackground": {
        backgroundColor: "hsl(var(--muted))",
    },
    ".cm-gutters": {
        backgroundColor: "hsl(var(--muted) / 0.35)",
        color: "hsl(var(--muted-foreground))",
        border: "none",
        borderRight: "1px solid hsl(var(--border))",
    },
    ".cm-activeLineGutter": {
        backgroundColor: "transparent",
    },
    ".cm-activeLine": {
        backgroundColor: "hsl(var(--muted) / 0.25)",
    },
});

function buildExtensions(): Extension[] {
    const exts: Extension[] = [
        minimalSetup,
        EditorView.lineWrapping,
        bracketMatching(),
        ndTheme,
        EditorView.updateListener.of((u) => {
            if (!u.docChanged || applyingRemote) return;
            emit("update:modelValue", u.state.doc.toString());
        }),
        EditorView.domEventHandlers({
            blur: () => {
                emit("blur");
                return false;
            },
        }),
    ];
    if (props.language === "json") {
        exts.push(json());
    } else if (props.language === "rhai") {
        exts.push(rhai());
    }
    return exts;
}

onMounted(() => {
    const el = host.value;
    if (!el) return;
    const state = EditorState.create({
        doc: props.modelValue ?? "",
        extensions: buildExtensions(),
    });
    viewRef.value = new EditorView({ state, parent: el });
});

onBeforeUnmount(() => {
    viewRef.value?.destroy();
    viewRef.value = null;
});

watch(
    () => props.modelValue,
    (v) => {
        const view = viewRef.value;
        if (!view) return;
        const next = v ?? "";
        if (view.state.doc.toString() === next) return;
        applyingRemote = true;
        view.dispatch({
            changes: {
                from: 0,
                to: view.state.doc.length,
                insert: next,
            },
        });
        applyingRemote = false;
    },
);
</script>

<template>
    <div
        :class="[
            'flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden',
            inputClass,
        ]"
    >
        <div ref="host" class="h-full min-h-0 w-full" />
    </div>
</template>
