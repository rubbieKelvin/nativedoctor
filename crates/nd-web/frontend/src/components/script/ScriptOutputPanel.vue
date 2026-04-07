<script setup lang="ts">
import { computed, ref } from "vue";
import { useEditorStore } from "@/stores/editor";
import { useExecutionStore } from "@/stores/execution";
import RuntimeEnvTable from "@/components/env/RuntimeEnvTable.vue";
import ScriptLogViewer from "@/components/script/ScriptLogViewer.vue";
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

const section = ref<"logs" | "env">("logs");
</script>

<template>
    <div
        class="flex h-full min-h-0 min-w-0 flex-col border-t border-border bg-muted/20"
    >
        <Tabs
            v-model="section"
            class="flex min-h-0 min-w-0 flex-1 flex-col gap-0 overflow-hidden"
        >
            <TabsList
                class="h-9 w-full shrink-0 justify-start rounded-none border-b border-border bg-background px-2"
            >
                <TabsTrigger value="logs" class="text-xs">Logs</TabsTrigger>
                <TabsTrigger value="env" class="text-xs"
                    >Runtime env</TabsTrigger
                >
            </TabsList>
            <TabsContent
                value="logs"
                class="mt-0 flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden"
            >
                <ScriptLogViewer
                    :logs="scriptLogs"
                    :error="scriptRunError"
                />
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
