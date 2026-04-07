<script setup lang="ts">
import { useEditorStore } from "@/stores/editor";
import { useExecutionStore } from "@/stores/execution";
import ScriptOutputPanel from "@/components/script/ScriptOutputPanel.vue";
import { Button } from "@/components/ui/button";
import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@/components/ui/resizable";
import { storeToRefs } from "pinia";

const editor = useEditorStore();
const execution = useExecutionStore();
const { scriptRaw } = storeToRefs(editor);
const { sending, sendErr } = storeToRefs(execution);
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
                <textarea
                    v-model="scriptRaw"
                    class="border-input bg-background focus-visible:ring-ring min-h-0 w-full flex-1 resize-none border-0 p-3 font-mono text-xs focus-visible:outline-none focus-visible:ring-1"
                    spellcheck="false"
                />
                <div
                    class="flex shrink-0 flex-wrap items-center gap-2 border-t border-border bg-background px-2 py-1.5"
                >
                    <Button
                        size="sm"
                        :disabled="sending"
                        @click="execution.doRunScript"
                    >
                        {{ sending ? "Running…" : "Run script" }}
                    </Button>
                    <span
                        v-if="sendErr"
                        class="text-xs text-destructive"
                        >{{ sendErr }}</span
                    >
                </div>
            </div>
        </ResizablePanel>
        <ResizableHandle with-handle />
        <ResizablePanel :default-size="40" :min-size="18">
            <div class="flex h-full min-h-0 min-w-0 flex-col overflow-hidden">
                <ScriptOutputPanel />
            </div>
        </ResizablePanel>
    </ResizablePanelGroup>
</template>
