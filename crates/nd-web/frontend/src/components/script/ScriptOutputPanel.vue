<script setup lang="ts">
import { computed, ref } from "vue";
import { useEditorStore } from "@/stores/editor";
import { useExecutionStore } from "@/stores/execution";
import RuntimeEnvTable from "@/components/env/RuntimeEnvTable.vue";
import ScriptLogViewer from "@/components/script/ScriptLogViewer.vue";
import ScriptRunTimeline from "@/components/script/timeline/ScriptRunTimeline.vue";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

const editor = useEditorStore();
const execution = useExecutionStore();

const activePath = computed(() => editor.activeTab?.path);

const scriptLogs = computed(() => {
    const p = activePath.value;
    if (!p) return [];
    return execution.scriptLogsByPath[p] ?? [];
});

const scriptRunError = computed(() => {
    const p = activePath.value;
    if (!p) return null;
    return execution.scriptRunErrorByPath[p] ?? null;
});

const scriptTimeline = computed(() => {
    const p = activePath.value;
    if (!p) return undefined;
    return execution.scriptTimelineByPath[p];
});

const section = ref<"logs" | "timeline" | "env">("timeline");
</script>

<template>
    <div class="flex h-full min-h-0 min-w-0 flex-col bg-background">
        <Tabs
            v-model="section"
            class="flex min-h-0 min-w-0 flex-1 flex-col gap-0 overflow-hidden"
        >
            <TabsList
                class="h-9 w-full shrink-0 justify-start rounded-none border-b border-border bg-background px-3"
            >
                <TabsTrigger value="timeline" class="text-sm font-medium">
                    Timeline
                </TabsTrigger>
                <TabsTrigger value="logs" class="text-sm font-medium">
                    Logs
                </TabsTrigger>
                <TabsTrigger value="env" class="text-sm font-medium">
                    Runtime env
                </TabsTrigger>
            </TabsList>
            <TabsContent
                value="timeline"
                class="mt-0 flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden"
            >
                <ScriptRunTimeline
                    :timeline="scriptTimeline"
                    :sending="execution.sending"
                    :run-error="scriptRunError"
                    :script-path="activePath"
                />
            </TabsContent>
            <TabsContent
                value="logs"
                class="mt-0 flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden"
            >
                <ScriptLogViewer :logs="scriptLogs" :error="scriptRunError" />
            </TabsContent>
            <TabsContent
                value="env"
                class="mt-0 flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden"
            >
                <RuntimeEnvTable />
            </TabsContent>
        </Tabs>
    </div>
</template>
