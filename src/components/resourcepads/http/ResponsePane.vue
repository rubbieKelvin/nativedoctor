<script setup lang="ts">
import { computed } from "vue";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import ResponseBody from "./ResponseBody.vue";
import ResponseCookies from "./ResponseCookies.vue";
import ResponseHeaders from "./ResponseHeaders.vue";

const props = defineProps<{
    status?: number;
    headers?: [string, string][];
    body?: string;
    durationMs?: number;
    error?: string;
}>();

const contentType = computed(() => {
    const header = props.headers?.find(
        ([name]) => name.toLowerCase() === "content-type",
    );
    return header?.[1] ?? "";
});
</script>

<template>
    <div
        v-if="(headers?.length || body != null) && !error"
        class="flex flex-1 flex-col h-full"
    >
        <Tabs default-value="headers" class="w-full h-full">
            <TabsList
                class="w-full rounded-none flex justify-baseline items-center"
            >
                <TabsTrigger class="request-tab-trigger" value="headers">
                    Headers
                </TabsTrigger>
                <TabsTrigger class="request-tab-trigger" value="body">
                    Body
                </TabsTrigger>
                <TabsTrigger class="request-tab-trigger" value="cookies">
                    Cookies
                </TabsTrigger>
                <div class="grow" />
                <div class="flex gap-2 pr-2 items-center">
                    <template v-if="status != null">
                        <span class="text-xs">
                            {{ status }}
                        </span>
                        <span>・</span>
                    </template>
                    <span
                        v-if="durationMs != null"
                        class="text-muted-foreground text-xs"
                    >
                        {{ durationMs }} ms
                    </span>
                </div>
            </TabsList>
            <div class="px-2 grow flex flex-col">
                <TabsContent value="headers" class="mt-2">
                    <ResponseHeaders :headers="headers ?? []" />
                </TabsContent>
                <TabsContent value="body" class="mt-2 flex">
                    <ResponseBody :body="body" :content-type="contentType" />
                </TabsContent>
                <TabsContent value="cookies" class="mt-2">
                    <ResponseCookies :headers="headers ?? []" />
                </TabsContent>
            </div>
        </Tabs>
    </div>
    <div v-else-if="error" class="p-3">
        <p class="text-destructive text-sm">
            {{ error }}
        </p>
    </div>
</template>
