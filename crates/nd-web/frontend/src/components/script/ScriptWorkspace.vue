<script setup lang="ts">
import { computed } from "vue";
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
import CodeMirrorEditor from "@/components/editor/CodeMirrorEditor.vue";

const editor = useEditorStore();
const execution = useExecutionStore();
const { scriptRaw, scriptDirty, scriptSaveError, savingScript } =
    storeToRefs(editor);
const { sending } = storeToRefs(execution);

const activePath = computed(() => editor.activeTab?.path);

const sendErr = computed(() => {
    const p = activePath.value;
    if (!p) return null;
    return execution.sendErrByPath[p] ?? null;
});
</script>

<template>
    <ResizablePanelGroup
        id="nd-script-workspace"
        direction="horizontal"
        auto-save-id="nd-script-workspace"
        class="h-full min-h-0 min-w-0 flex-1"
    >
        <ResizablePanel :default-size="30" :min-size="25">
            <div class="flex h-full min-h-0 min-w-0 flex-col overflow-hidden">
                <CodeMirrorEditor
                    :key="activePath ?? ''"
                    v-model="scriptRaw"
                    language="rhai"
                    class="min-h-0 flex-1"
                />
                <div
                    class="flex shrink-0 flex-wrap items-center gap-2 border-t border-border bg-background px-2 py-1.5"
                >
                    <Button
                        size="sm"
                        variant="secondary"
                        :disabled="
                            !scriptDirty ||
                            sending ||
                            savingScript
                        "
                        @click="editor.saveActiveScript"
                    >
                        {{
                            savingScript
                                ? "Saving…"
                                : scriptDirty
                                  ? "Save"
                                  : "Saved"
                        }}
                    </Button>
                    <Button
                        size="sm"
                        :disabled="sending || savingScript"
                        @click="execution.doRunScript"
                    >
                        {{ sending ? "Running…" : "Run script" }}
                    </Button>
                    <span
                        v-if="scriptSaveError"
                        class="text-destructive text-xs"
                        >{{ scriptSaveError }}</span
                    >
                    <span v-if="sendErr" class="text-xs text-destructive">{{
                        sendErr
                    }}</span>
                </div>
            </div>
        </ResizablePanel>
        <ResizableHandle with-handle />
        <ResizablePanel :default-size="70" :min-size="18">
            <div class="flex h-full min-h-0 min-w-0 flex-col overflow-hidden">
                <ScriptOutputPanel />
            </div>
        </ResizablePanel>
    </ResizablePanelGroup>
</template>
