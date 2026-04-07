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
            class="h-9 w-full shrink-0 justify-start rounded-none border-b border-border bg-background px-2"
        >
            <TabsTrigger value="params" class="text-xs">Params</TabsTrigger>
            <TabsTrigger value="headers" class="text-xs">Headers</TabsTrigger>
            <TabsTrigger value="body" class="text-xs">Body</TabsTrigger>
            <TabsTrigger value="input" class="text-xs">Input</TabsTrigger>
            <TabsTrigger value="auth" class="text-xs">Auth</TabsTrigger>
        </TabsList>

        <div class="min-h-0 min-w-0 flex-1 overflow-auto p-2">
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
                <p class="text-muted-foreground text-xs">
                    Use <strong>Headers</strong> for
                    <code class="rounded bg-muted px-1">Authorization</code>, or
                    reference runtime variables in the URL/body.
                </p>
            </TabsContent>
        </div>
    </Tabs>
</template>
