<script setup lang="ts">
import type { AppModel } from "@/composables/useAppModel";
import KeyValueEditor from "@/components/request/KeyValueEditor.vue";
import RequestBodyEditor from "@/components/request/RequestBodyEditor.vue";
import { Card, CardContent } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

defineProps<{
    app: AppModel;
}>();
</script>

<template>
    <Tabs
        v-model="app.reqSubTab"
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
                v-if="app.activeTab?.parseError"
                class="mb-2 text-sm text-destructive"
            >
                {{ app.activeTab.parseError }} — fix document
            </p>

            <TabsContent value="params" class="mt-0">
                <KeyValueEditor v-model="app.queryRecord" />
            </TabsContent>

            <TabsContent value="headers" class="mt-0">
                <KeyValueEditor v-model="app.headersRecord" />
            </TabsContent>

            <TabsContent value="body" class="mt-0">
                <RequestBodyEditor :app="app" />
            </TabsContent>

            <TabsContent value="input" class="mt-0">
                <p
                    v-if="app.overridesJsonError"
                    class="text-sm text-destructive"
                >
                    {{ app.overridesJsonError }}
                </p>
                <KeyValueEditor v-model="app.overridesRecord" />
            </TabsContent>

            <TabsContent value="auth" class="mt-0">
                <p class="text-muted-foreground text-xs">
                    Use <strong>Headers</strong> for
                    <code class="rounded bg-muted px-1">Authorization</code>, or
                    reference runtime variables in the URL/body.
                </p>
            </TabsContent>
        </div>
    </Tabs>
</template>
