<script setup lang="ts">
import type { AppModel } from "@/composables/useAppModel";
import RequestToolbar from "@/components/request/RequestToolbar.vue";
import RequestEditorTabs from "@/components/request/RequestEditorTabs.vue";
import ResponsePanel from "@/components/response/ResponsePanel.vue";
import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@/components/ui/resizable";

defineProps<{
    app: AppModel;
}>();
</script>

<template>
    <ResizablePanelGroup
        id="nd-request-workspace"
        direction="vertical"
        auto-save-id="nd-request-workspace"
        class="h-full min-h-0 min-w-0 flex-1"
    >
        <ResizablePanel :default-size="50" :min-size="28">
            <div class="flex h-full min-h-0 min-w-0 flex-col">
                <RequestToolbar :app="app" />
                <RequestEditorTabs :app="app" />
            </div>
        </ResizablePanel>
        <ResizableHandle with-handle />
        <ResizablePanel :default-size="50" :min-size="22">
            <div class="flex h-full min-h-0 min-w-0 flex-col overflow-hidden">
                <ResponsePanel :app="app" />
            </div>
        </ResizablePanel>
    </ResizablePanelGroup>
</template>
