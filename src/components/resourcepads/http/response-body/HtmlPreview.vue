<script setup lang="ts">
import { ref, onMounted, watch } from "vue";

const props = defineProps<{
    body: string;
}>();

const iframeRef = ref<HTMLIFrameElement | null>(null);

function writeDoc() {
    const iframe = iframeRef.value;
    if (!iframe?.contentDocument) return;
    iframe.contentDocument.open();
    iframe.contentDocument.write(props.body ?? "");
    iframe.contentDocument.close();
}

onMounted(writeDoc);
watch(() => props.body, writeDoc);
</script>

<template>
    <div class="flex-1 flex flex-col min-h-0">
        <iframe
            ref="iframeRef"
            title="HTML preview"
            class="w-full flex-1 min-h-[200px] rounded border border-border bg-white"
            sandbox="allow-same-origin"
        />
    </div>
</template>
