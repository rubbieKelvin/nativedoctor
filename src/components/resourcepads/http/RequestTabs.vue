<script setup lang="ts">
import type { HttpBody } from "@/shared/types/resources";
import type { KeyValue } from "./types";
import BodyPanel from "./BodyPanel.vue";
import HeadersPanel from "./HeadersPanel.vue";
import ParamsPanel from "./ParamsPanel.vue";
import AuthPanel from "./AuthPanel.vue";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

defineProps<{
    params: KeyValue[];
    headers: KeyValue[];
    body: HttpBody;
    bodyDisabled?: boolean;
}>();

const emit = defineEmits<{
    (e: "update:params", value: KeyValue[]): void;
    (e: "update:headers", value: KeyValue[]): void;
    (e: "update:body", value: HttpBody): void;
}>();
</script>

<template>
    <Tabs default-value="params" class="w-full">
        <TabsList class="w-full rounded-none flex items-start justify-baseline">
            <TabsTrigger class="request-tab-trigger" value="params">
                Params
            </TabsTrigger>
            <TabsTrigger class="request-tab-trigger" value="headers">
                Headers
            </TabsTrigger>
            <TabsTrigger class="request-tab-trigger" value="body">
                Body
            </TabsTrigger>
            <TabsTrigger class="request-tab-trigger" value="auth">
                Auth
            </TabsTrigger>
        </TabsList>
        <div class="px-2">
            <TabsContent value="params" class="mt-2">
                <ParamsPanel
                    :model-value="params"
                    @update:model-value="emit('update:params', $event)"
                />
            </TabsContent>
            <TabsContent value="headers" class="mt-2">
                <HeadersPanel
                    :model-value="headers"
                    @update:model-value="emit('update:headers', $event)"
                />
            </TabsContent>
            <TabsContent value="body" class="mt-2">
                <BodyPanel
                    :model-value="body"
                    :disabled="bodyDisabled"
                    @update:model-value="emit('update:body', $event as HttpBody)"
                />
            </TabsContent>
            <TabsContent value="auth" class="mt-2">
                <AuthPanel />
            </TabsContent>
        </div>
    </Tabs>
</template>
