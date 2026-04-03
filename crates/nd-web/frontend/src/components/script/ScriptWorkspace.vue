<script setup lang="ts">
import type { AppModel } from "@/composables/useAppModel";
import ScriptOutputPanel from "@/components/script/ScriptOutputPanel.vue";
import { Button } from "@/components/ui/button";
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
        id="nd-script-workspace"
        direction="vertical"
        auto-save-id="nd-script-workspace"
        class="h-full min-h-0 min-w-0 flex-1"
    >
        <ResizablePanel :default-size="60" :min-size="25">
            <div class="flex h-full min-h-0 min-w-0 flex-col overflow-hidden">
                <div
                    class="flex shrink-0 flex-wrap items-center gap-2 border-b border-border bg-background px-2 py-1.5"
                >
                    <Button
                        size="sm"
                        :disabled="app.sending"
                        @click="app.doRunScript"
                    >
                        {{ app.sending ? "Running…" : "Run script" }}
                    </Button>
                    <span
                        v-if="app.sendErr"
                        class="text-xs text-destructive"
                        >{{ app.sendErr }}</span
                    >
                </div>
                <textarea
                    v-model="app.scriptRaw"
                    class="border-input bg-background focus-visible:ring-ring min-h-0 w-full flex-1 resize-none border-0 p-3 font-mono text-xs focus-visible:outline-none focus-visible:ring-1"
                    spellcheck="false"
                />
            </div>
        </ResizablePanel>
        <ResizableHandle with-handle />
        <ResizablePanel :default-size="40" :min-size="18">
            <div class="flex h-full min-h-0 min-w-0 flex-col overflow-hidden">
                <ScriptOutputPanel :app="app" />
            </div>
        </ResizablePanel>
    </ResizablePanelGroup>
</template>
