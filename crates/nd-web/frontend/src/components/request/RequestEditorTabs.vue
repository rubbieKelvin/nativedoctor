<script setup lang="ts">
import { useEditorStore } from "@/stores/editor";
import KeyValueEditor from "@/components/request/KeyValueEditor.vue";
import RequestBodyEditor from "@/components/request/RequestBodyEditor.vue";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { storeToRefs } from "pinia";

const editor = useEditorStore();
const {
    reqSubTab,
    activeTab,
    queryRecord,
    headersRecord,
    overridesRecord,
    overridesJsonError,
} = storeToRefs(editor);
</script>

<template>
    <Tabs
        v-model="reqSubTab"
        class="flex min-h-0 min-w-0 flex-1 flex-col gap-0"
    >
        <TabsList
            class="h-9 w-full shrink-0 justify-start rounded-none border-b border-border bg-background px-3"
        >
            <TabsTrigger value="params" class="text-sm font-medium">
                Params
            </TabsTrigger>
            <TabsTrigger value="headers" class="text-sm font-medium">
                Headers
            </TabsTrigger>
            <TabsTrigger value="body" class="text-sm font-medium">
                Body
            </TabsTrigger>
            <TabsTrigger value="input" class="text-sm font-medium">
                Input
            </TabsTrigger>
            <TabsTrigger value="auth" class="text-sm font-medium">
                Auth
            </TabsTrigger>
        </TabsList>

        <div class="min-h-0 min-w-0 flex-1 overflow-auto p-3">
            <p
                v-if="activeTab?.parseError"
                class="mb-2 text-sm text-destructive"
            >
                {{ activeTab.parseError }} — fix document
            </p>

            <TabsContent value="params" class="mt-0 h-full">
                <KeyValueEditor v-model="queryRecord" />
            </TabsContent>

            <TabsContent value="headers" class="mt-0 h-full">
                <KeyValueEditor v-model="headersRecord" />
            </TabsContent>

            <TabsContent value="body" class="mt-0 h-full">
                <RequestBodyEditor />
            </TabsContent>

            <TabsContent value="input" class="mt-0 h-full">
                <p
                    v-if="overridesJsonError"
                    class="text-sm text-destructive"
                >
                    {{ overridesJsonError }}
                </p>
                <KeyValueEditor v-model="overridesRecord" />
            </TabsContent>

            <TabsContent value="auth" class="mt-0 h-full">
                <p class="text-muted-foreground text-sm leading-relaxed">
                    Use <strong class="text-foreground">Headers</strong> for
                    <code class="rounded bg-muted px-1 font-mono text-xs"
                        >Authorization</code
                    >, or reference runtime variables in the URL/body.
                </p>
            </TabsContent>
        </div>
    </Tabs>
</template>
